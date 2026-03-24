package grpcapi

import (
	"fmt"
	"log"
	"net"

	orchpb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/orchestration/v1"
	feedpb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/feed/v1"
	mailpb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/mail/v1"
	"google.golang.org/grpc"
)

// GRPCServer wraps a gRPC server and all registered service implementations.
type GRPCServer struct {
	port     int
	server   *grpc.Server
	listener net.Listener

	// Service implementations (exposed for testing)
	Orchestration *OrchestrationHandler
	Health        *HealthHandler
	Feed          *FeedHandler
	Mail          *MailHandler
}

// NewServer creates a new GRPCServer that will listen on the given port.
// All handlers share the same InMemoryEngine (standalone mode).
func NewServer(port int) *GRPCServer {
	engine := NewInMemoryEngine()
	return NewServerWithEngine(port, engine)
}

// NewServerWithEngine creates a new GRPCServer backed by the provided Engine.
// All handlers share the same engine instance.
func NewServerWithEngine(port int, engine Engine) *GRPCServer {
	s := &GRPCServer{
		port:          port,
		server:        grpc.NewServer(),
		Orchestration: NewOrchestrationHandler(engine),
		Health:        NewHealthHandler(),
		Feed:          NewFeedHandler(engine),
		Mail:          NewMailHandler(engine),
	}

	// Register all services.
	orchpb.RegisterHealthServiceServer(s.server, s.Health)
	orchpb.RegisterOrchestrationServiceServer(s.server, s.Orchestration)
	feedpb.RegisterFeedServiceServer(s.server, s.Feed)
	mailpb.RegisterMailServiceServer(s.server, s.Mail)

	return s
}

// Start begins listening and serving gRPC requests. It blocks until the server
// is stopped or encounters a fatal error.
func (s *GRPCServer) Start() error {
	addr := fmt.Sprintf(":%d", s.port)
	lis, err := net.Listen("tcp", addr)
	if err != nil {
		return fmt.Errorf("grpcapi: listen on %s: %w", addr, err)
	}
	s.listener = lis
	log.Printf("grpcapi: serving on %s", addr)
	return s.server.Serve(lis)
}

// StartOnListener starts the gRPC server on the provided listener.
// This is useful for tests that need a pre-bound listener.
func (s *GRPCServer) StartOnListener(lis net.Listener) error {
	s.listener = lis
	return s.server.Serve(lis)
}

// Stop performs a graceful shutdown of the gRPC server.
func (s *GRPCServer) Stop() {
	log.Println("grpcapi: shutting down")
	s.server.GracefulStop()
}

// Addr returns the listener address, or empty string if not yet started.
func (s *GRPCServer) Addr() string {
	if s.listener != nil {
		return s.listener.Addr().String()
	}
	return ""
}
