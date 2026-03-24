package grpcapi

import (
	"context"
	"time"

	"github.com/google/uuid"
	pb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/feed/v1"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/timestamppb"
)

// FeedHandler implements the FeedService gRPC service.
// It delegates to an Engine for event storage and retrieval.
type FeedHandler struct {
	pb.UnimplementedFeedServiceServer

	engine Engine
}

// NewFeedHandler returns a new FeedHandler backed by the given Engine.
// If engine is nil, a default InMemoryEngine is used.
func NewFeedHandler(engine ...Engine) *FeedHandler {
	var e Engine
	if len(engine) > 0 && engine[0] != nil {
		e = engine[0]
	} else {
		e = NewInMemoryEngine()
	}
	return &FeedHandler{engine: e}
}

// SubscriberCount returns the current number of subscribers.
// This uses a simple heuristic - we can't easily expose subscriber count
// through the Engine interface, so this is only accurate for InMemoryEngine.
func (f *FeedHandler) SubscriberCount() int {
	if ime, ok := f.engine.(*InMemoryEngine); ok {
		ime.subMu.Lock()
		defer ime.subMu.Unlock()
		return len(ime.subs)
	}
	if de, ok := f.engine.(*DaemonEngine); ok {
		de.fallback.subMu.Lock()
		defer de.fallback.subMu.Unlock()
		return len(de.fallback.subs)
	}
	return 0
}

// Publish adds an event and fans it out to all stream subscribers.
// It is safe for concurrent use.
func (f *FeedHandler) Publish(evt *pb.FeedEvent) {
	info := &EventInfo{
		ID:        evt.GetId(),
		EventType: evt.GetEventType(),
		Source:    evt.GetSource(),
		RigID:    evt.GetRigId(),
		AgentID:  evt.GetAgentId(),
		Summary:  evt.GetSummary(),
		Severity: evt.GetSeverity(),
	}
	if evt.GetCreatedAt() != nil {
		info.CreatedAt = evt.GetCreatedAt().AsTime()
	}

	// Publish through the engine which handles storage and fan-out.
	if ime, ok := f.engine.(*InMemoryEngine); ok {
		ime.PublishEvent(info)
	} else if de, ok := f.engine.(*DaemonEngine); ok {
		de.PublishEvent(info)
	}
}

// StreamEvents streams feed events to the client. It sends a heartbeat every
// 5 seconds and any events that are published while the stream is open.
func (f *FeedHandler) StreamEvents(req *pb.StreamEventsRequest, stream grpc.ServerStreamingServer[pb.FeedEvent]) error {
	ch, cancel := f.engine.SubscribeEvents()
	defer cancel()

	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-stream.Context().Done():
			return stream.Context().Err()
		case evt := <-ch:
			// Apply optional filters.
			if req.GetRigId() != "" && evt.RigID != req.GetRigId() {
				continue
			}
			if err := stream.Send(eventInfoToProto(evt)); err != nil {
				return err
			}
		case <-ticker.C:
			heartbeat := &pb.FeedEvent{
				Id:        uuid.NewString(),
				EventType: "heartbeat",
				Source:    "system",
				Summary:   "heartbeat",
				Severity:  "debug",
				CreatedAt: timestamppb.Now(),
			}
			if err := stream.Send(heartbeat); err != nil {
				return err
			}
		}
	}
}

// GetRecentEvents returns recent events.
func (f *FeedHandler) GetRecentEvents(_ context.Context, req *pb.GetRecentEventsRequest) (*pb.EventsResponse, error) {
	// Fetch a large window from the engine (up to ring buffer size).
	// Pagination (offset/limit) is applied here in the handler.
	const maxFetch = 1024
	events, err := f.engine.GetRecentEvents(maxFetch)
	if err != nil {
		return nil, err
	}

	// Apply rig filter.
	var filtered []*EventInfo
	for _, evt := range events {
		if req.GetRigId() != "" && evt.RigID != req.GetRigId() {
			continue
		}
		filtered = append(filtered, evt)
	}

	total := int32(len(filtered))

	// Apply offset / limit.
	offset := int(req.GetOffset())
	limit := int(req.GetLimit())
	if limit <= 0 {
		limit = 50
	}
	if offset > len(filtered) {
		offset = len(filtered)
	}
	end := offset + limit
	if end > len(filtered) {
		end = len(filtered)
	}
	page := filtered[offset:end]

	var pbEvents []*pb.FeedEvent
	for _, evt := range page {
		pbEvents = append(pbEvents, eventInfoToProto(evt))
	}

	return &pb.EventsResponse{
		Events: pbEvents,
		Total:  total,
	}, nil
}

// eventInfoToProto converts an EventInfo to a protobuf FeedEvent.
func eventInfoToProto(e *EventInfo) *pb.FeedEvent {
	evt := &pb.FeedEvent{
		Id:        e.ID,
		EventType: e.EventType,
		Source:    e.Source,
		RigId:    e.RigID,
		AgentId:  e.AgentID,
		Summary:  e.Summary,
		Severity: e.Severity,
		CreatedAt: timestamppb.New(e.CreatedAt),
	}
	if evt.Id == "" {
		evt.Id = uuid.NewString()
	}
	return evt
}
