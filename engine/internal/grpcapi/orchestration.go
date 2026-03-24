package grpcapi

import (
	"context"

	"github.com/google/uuid"
	pb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/orchestration/v1"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
)

// HealthHandler implements the HealthService gRPC service.
type HealthHandler struct {
	pb.UnimplementedHealthServiceServer
}

// NewHealthHandler returns a new HealthHandler.
func NewHealthHandler() *HealthHandler {
	return &HealthHandler{}
}

// Check returns the serving status of the engine.
func (h *HealthHandler) Check(_ context.Context, _ *pb.HealthCheckRequest) (*pb.HealthCheckResponse, error) {
	return &pb.HealthCheckResponse{
		Status: pb.HealthCheckResponse_SERVING_STATUS_SERVING,
	}, nil
}

// OrchestrationHandler implements the OrchestrationService gRPC service.
// It delegates to an Engine for all operations.
type OrchestrationHandler struct {
	pb.UnimplementedOrchestrationServiceServer

	engine Engine
}

// NewOrchestrationHandler returns a new OrchestrationHandler backed by the
// given Engine. If engine is nil, a default InMemoryEngine is used.
func NewOrchestrationHandler(engine ...Engine) *OrchestrationHandler {
	var e Engine
	if len(engine) > 0 && engine[0] != nil {
		e = engine[0]
	} else {
		e = NewInMemoryEngine()
	}
	return &OrchestrationHandler{engine: e}
}

// Engine returns the underlying Engine (useful for testing and server wiring).
func (o *OrchestrationHandler) Engine() Engine {
	return o.engine
}

// --- Town lifecycle ---

func (o *OrchestrationHandler) CreateTown(_ context.Context, req *pb.CreateTownRequest) (*pb.Town, error) {
	now := timestamppb.Now()
	return &pb.Town{
		Id:        uuid.NewString(),
		Name:      req.GetName(),
		Owner:     req.GetOwner(),
		CreatedAt: now,
		UpdatedAt: now,
	}, nil
}

func (o *OrchestrationHandler) GetTown(_ context.Context, req *pb.GetTownRequest) (*pb.Town, error) {
	if req.GetId() == "" {
		return nil, status.Error(codes.InvalidArgument, "id is required")
	}
	info, err := o.engine.GetTownConfig()
	if err != nil {
		return nil, status.Errorf(codes.Internal, "getting town config: %v", err)
	}
	return &pb.Town{
		Id:        info.ID,
		Name:      info.Name,
		Owner:     info.Owner,
		CreatedAt: timestamppb.New(info.CreatedAt),
		UpdatedAt: timestamppb.Now(),
	}, nil
}

// --- Rig management ---

func (o *OrchestrationHandler) CreateRig(_ context.Context, req *pb.CreateRigRequest) (*pb.Rig, error) {
	now := timestamppb.Now()
	return &pb.Rig{
		Id:          uuid.NewString(),
		TownId:      req.GetTownId(),
		Name:        req.GetName(),
		RepoUrl:     req.GetRepoUrl(),
		BeadsPrefix: req.GetBeadsPrefix(),
		CreatedAt:   now,
		UpdatedAt:   now,
	}, nil
}

func (o *OrchestrationHandler) ListRigs(_ context.Context, _ *pb.ListRigsRequest) (*pb.ListRigsResponse, error) {
	rigs, err := o.engine.ListRigs()
	if err != nil {
		return nil, status.Errorf(codes.Internal, "listing rigs: %v", err)
	}
	var pbRigs []*pb.Rig
	for _, r := range rigs {
		pbRigs = append(pbRigs, &pb.Rig{
			Id:          r.Name,
			Name:        r.Name,
			RepoUrl:     r.GitURL,
			BeadsPrefix: r.Prefix,
		})
	}
	return &pb.ListRigsResponse{Rigs: pbRigs}, nil
}

// --- Agent lifecycle ---

func (o *OrchestrationHandler) SpawnAgent(_ context.Context, req *pb.SpawnAgentRequest) (*pb.Agent, error) {
	info, err := o.engine.SpawnAgent(
		req.GetName(),
		req.GetRole(),
		req.GetRigId(),
		req.GetRuntime(),
		req.GetConfigJson(),
	)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "spawning agent: %v", err)
	}
	return agentInfoToProto(info), nil
}

func (o *OrchestrationHandler) KillAgent(_ context.Context, req *pb.KillAgentRequest) (*emptypb.Empty, error) {
	if err := o.engine.KillAgent(req.GetId()); err != nil {
		return nil, status.Errorf(codes.NotFound, "agent %q not found", req.GetId())
	}
	return &emptypb.Empty{}, nil
}

func (o *OrchestrationHandler) GetAgentStatus(_ context.Context, req *pb.GetAgentStatusRequest) (*pb.Agent, error) {
	info, err := o.engine.GetAgent(req.GetId())
	if err != nil {
		return nil, status.Errorf(codes.NotFound, "agent %q not found", req.GetId())
	}
	return agentInfoToProto(info), nil
}

func (o *OrchestrationHandler) ListAgents(_ context.Context, req *pb.ListAgentsRequest) (*pb.ListAgentsResponse, error) {
	agents, err := o.engine.ListAgents(
		req.GetRigId(),
		req.GetRole(),
		req.GetStatus(),
	)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "listing agents: %v", err)
	}
	var pbAgents []*pb.Agent
	for _, a := range agents {
		pbAgents = append(pbAgents, agentInfoToProto(a))
	}
	return &pb.ListAgentsResponse{Agents: pbAgents}, nil
}

// --- Convoy operations ---

func (o *OrchestrationHandler) StartConvoy(_ context.Context, req *pb.StartConvoyRequest) (*pb.Convoy, error) {
	info, err := o.engine.StartConvoy(
		req.GetName(),
		req.GetFormula(),
		req.GetConfigJson(),
	)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "starting convoy: %v", err)
	}
	return convoyInfoToProto(info), nil
}

func (o *OrchestrationHandler) GetConvoyStatus(_ context.Context, req *pb.GetConvoyStatusRequest) (*pb.Convoy, error) {
	if req.GetId() == "" {
		return nil, status.Error(codes.InvalidArgument, "id is required")
	}
	info, err := o.engine.GetConvoy(req.GetId())
	if err != nil {
		return nil, status.Errorf(codes.NotFound, "convoy %q not found", req.GetId())
	}
	return convoyInfoToProto(info), nil
}

func (o *OrchestrationHandler) ListConvoys(_ context.Context, req *pb.ListConvoysRequest) (*pb.ListConvoysResponse, error) {
	convoys, err := o.engine.ListConvoys(req.GetStatus())
	if err != nil {
		return nil, status.Errorf(codes.Internal, "listing convoys: %v", err)
	}
	var pbConvoys []*pb.Convoy
	for _, c := range convoys {
		pbConvoys = append(pbConvoys, convoyInfoToProto(c))
	}
	return &pb.ListConvoysResponse{Convoys: pbConvoys}, nil
}

// --- Work dispatch ---

func (o *OrchestrationHandler) DispatchWork(_ context.Context, req *pb.DispatchWorkRequest) (*pb.DispatchWorkResponse, error) {
	return &pb.DispatchWorkResponse{
		WorkItemId: uuid.NewString(),
		Accepted:   true,
	}, nil
}

// --- Merge queue ---

func (o *OrchestrationHandler) QueueMerge(_ context.Context, req *pb.QueueMergeRequest) (*pb.MergeRequest, error) {
	info, err := o.engine.QueueMerge(
		req.GetRigId(),
		req.GetBranch(),
		req.GetTargetBranch(),
		req.GetWorkItemId(),
		req.GetAgentId(),
	)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "queueing merge: %v", err)
	}
	return mergeInfoToProto(info), nil
}

func (o *OrchestrationHandler) GetMergeQueue(_ context.Context, req *pb.GetMergeQueueRequest) (*pb.MergeQueueResponse, error) {
	items, err := o.engine.GetMergeQueue(req.GetRigId())
	if err != nil {
		return nil, status.Errorf(codes.Internal, "getting merge queue: %v", err)
	}
	var pbItems []*pb.MergeRequest
	for _, item := range items {
		pbItems = append(pbItems, mergeInfoToProto(item))
	}
	return &pb.MergeQueueResponse{Requests: pbItems}, nil
}

// --- Proto conversion helpers ---

func agentInfoToProto(a *AgentInfo) *pb.Agent {
	return &pb.Agent{
		Id:             a.ID,
		Name:           a.Name,
		Role:           a.Role,
		RigId:          a.RigID,
		Status:         a.Status,
		Runtime:        a.Runtime,
		ConfigJson:     a.ConfigJSON,
		LastActivityAt: timestamppb.New(a.LastActivityAt),
		CreatedAt:      timestamppb.New(a.CreatedAt),
	}
}

func convoyInfoToProto(c *ConvoyInfo) *pb.Convoy {
	return &pb.Convoy{
		Id:         c.ID,
		Name:       c.Name,
		Status:     c.Status,
		Formula:    c.Formula,
		ConfigJson: c.ConfigJSON,
		CreatedAt:  timestamppb.New(c.CreatedAt),
		UpdatedAt:  timestamppb.New(c.UpdatedAt),
	}
}

func mergeInfoToProto(m *MergeInfo) *pb.MergeRequest {
	return &pb.MergeRequest{
		Id:           m.ID,
		WorkItemId:   m.WorkItemID,
		RigId:        m.RigID,
		Branch:       m.Branch,
		TargetBranch: m.TargetBranch,
		Status:       m.Status,
		AgentId:      m.AgentID,
		QueuedAt:     timestamppb.New(m.QueuedAt),
	}
}
