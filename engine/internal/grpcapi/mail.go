package grpcapi

import (
	"context"

	pb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/mail/v1"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
)

// MailHandler implements the MailService gRPC service.
// It delegates to an Engine for all operations.
type MailHandler struct {
	pb.UnimplementedMailServiceServer

	engine Engine
}

// NewMailHandler returns a new MailHandler backed by the given Engine.
// If engine is nil, a default InMemoryEngine is used.
func NewMailHandler(engine ...Engine) *MailHandler {
	var e Engine
	if len(engine) > 0 && engine[0] != nil {
		e = engine[0]
	} else {
		e = NewInMemoryEngine()
	}
	return &MailHandler{engine: e}
}

// SendMail creates a new mail message.
func (m *MailHandler) SendMail(_ context.Context, req *pb.SendMailRequest) (*pb.MailMessage, error) {
	msg := &MailInfo{
		FromAddr:    req.GetFromAddr(),
		ToAddr:      req.GetToAddr(),
		Subject:     req.GetSubject(),
		Body:        req.GetBody(),
		Priority:    req.GetPriority(),
		MessageType: req.GetMessageType(),
		Delivery:    req.GetDelivery(),
		ThreadID:    req.GetThreadId(),
		ReplyTo:     req.GetReplyTo(),
		Channel:     req.GetChannel(),
	}

	result, err := m.engine.SendMail(msg)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "sending mail: %v", err)
	}

	return mailInfoToProto(result), nil
}

// GetInbox returns messages matching the filter criteria.
func (m *MailHandler) GetInbox(_ context.Context, req *pb.GetInboxRequest) (*pb.InboxResponse, error) {
	msgs, total, err := m.engine.GetInbox(
		req.GetToAddr(),
		int(req.GetLimit()),
		int(req.GetOffset()),
		req.GetUnreadOnly(),
	)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "reading inbox: %v", err)
	}

	var pbMsgs []*pb.MailMessage
	for _, msg := range msgs {
		pbMsgs = append(pbMsgs, mailInfoToProto(msg))
	}

	return &pb.InboxResponse{
		Messages: pbMsgs,
		Total:    int32(total),
	}, nil
}

// MarkRead sets the read flag on a message.
func (m *MailHandler) MarkRead(_ context.Context, req *pb.MarkReadRequest) (*emptypb.Empty, error) {
	if err := m.engine.MarkRead(req.GetId()); err != nil {
		return nil, status.Errorf(codes.NotFound, "message %q not found", req.GetId())
	}
	return &emptypb.Empty{}, nil
}

// ClaimQueueMessage finds the oldest unclaimed message in the given queue and
// marks it as claimed by the requestor.
//
// Note: This is a stateful operation that doesn't map cleanly to the Engine
// interface. For Phase 1, it uses the Engine's GetInbox to find messages and
// then performs the claim. A proper implementation would add ClaimMessage to
// the Engine interface.
func (m *MailHandler) ClaimQueueMessage(_ context.Context, req *pb.ClaimRequest) (*pb.MailMessage, error) {
	// Claiming requires in-memory state mutation which the InMemoryEngine
	// handles internally. For the Engine interface, we do a simple inbox
	// lookup + mark as claimed via the underlying engine.
	//
	// This is a pragmatic compromise: the in-memory engine stores claim
	// state in its MailInfo. DaemonEngine would need beads claim support.
	msgs, _, err := m.engine.GetInbox("", 1000, 0, false)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "reading messages: %v", err)
	}

	for _, msg := range msgs {
		if msg.Queue != req.GetQueue() {
			continue
		}
		if msg.ClaimedBy != "" {
			continue
		}
		// Found an unclaimed message. Mark it as claimed.
		// Since we can't directly mutate through the Engine interface,
		// we set the fields and trust the reference is shared (InMemoryEngine).
		msg.ClaimedBy = req.GetClaimedBy()
		return mailInfoToProto(msg), nil
	}

	return nil, status.Errorf(codes.NotFound, "no unclaimed messages in queue %q", req.GetQueue())
}

// --- Proto conversion helper ---

func mailInfoToProto(m *MailInfo) *pb.MailMessage {
	pbMsg := &pb.MailMessage{
		Id:          m.ID,
		FromAddr:    m.FromAddr,
		ToAddr:      m.ToAddr,
		Subject:     m.Subject,
		Body:        m.Body,
		Priority:    m.Priority,
		MessageType: m.MessageType,
		Delivery:    m.Delivery,
		ThreadId:    m.ThreadID,
		ReplyTo:     m.ReplyTo,
		Channel:     m.Channel,
		Queue:       m.Queue,
		Read:        m.Read,
		ClaimedBy:   m.ClaimedBy,
		CreatedAt:   timestamppb.New(m.CreatedAt),
	}
	if !m.ClaimedAt.IsZero() {
		pbMsg.ClaimedAt = timestamppb.New(m.ClaimedAt)
	}
	return pbMsg
}
