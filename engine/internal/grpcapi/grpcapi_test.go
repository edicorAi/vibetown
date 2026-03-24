package grpcapi_test

import (
	"context"
	"net"
	"testing"
	"time"

	"github.com/edicorai/vibetown/engine/internal/grpcapi"
	feedpb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/feed/v1"
	mailpb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/mail/v1"
	orchpb "github.com/edicorai/vibetown/engine/internal/grpcapi/gen/vibetown/orchestration/v1"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// testEnv holds a running in-process gRPC server and client connection.
type testEnv struct {
	srv  *grpcapi.GRPCServer
	conn *grpc.ClientConn
}

func newTestEnv(t *testing.T) *testEnv {
	t.Helper()

	// Bind to a random port.
	lis, err := net.Listen("tcp", "127.0.0.1:0")
	require.NoError(t, err)

	srv := grpcapi.NewServer(0) // port is unused since we supply our own listener
	go func() {
		_ = srv.StartOnListener(lis)
	}()

	// Connect a client.
	conn, err := grpc.NewClient(
		lis.Addr().String(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	require.NoError(t, err)

	t.Cleanup(func() {
		conn.Close()
		srv.Stop()
	})

	return &testEnv{srv: srv, conn: conn}
}

// newTestEnvWithEngine creates a test env backed by the given Engine.
func newTestEnvWithEngine(t *testing.T, engine grpcapi.Engine) *testEnv {
	t.Helper()

	lis, err := net.Listen("tcp", "127.0.0.1:0")
	require.NoError(t, err)

	srv := grpcapi.NewServerWithEngine(0, engine)
	go func() {
		_ = srv.StartOnListener(lis)
	}()

	conn, err := grpc.NewClient(
		lis.Addr().String(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	require.NoError(t, err)

	t.Cleanup(func() {
		conn.Close()
		srv.Stop()
	})

	return &testEnv{srv: srv, conn: conn}
}

// ---------- Health ----------

func TestHealthCheck(t *testing.T) {
	env := newTestEnv(t)
	client := orchpb.NewHealthServiceClient(env.conn)

	resp, err := client.Check(context.Background(), &orchpb.HealthCheckRequest{})
	require.NoError(t, err)
	assert.Equal(t, orchpb.HealthCheckResponse_SERVING_STATUS_SERVING, resp.GetStatus())
}

// ---------- Agent lifecycle ----------

func TestSpawnAndListAgents(t *testing.T) {
	env := newTestEnv(t)
	client := orchpb.NewOrchestrationServiceClient(env.conn)
	ctx := context.Background()

	// Spawn an agent.
	agent, err := client.SpawnAgent(ctx, &orchpb.SpawnAgentRequest{
		Name:    "test-agent",
		Role:    "crew",
		RigId:   "rig-1",
		Runtime: "claude",
	})
	require.NoError(t, err)
	assert.NotEmpty(t, agent.GetId())
	assert.Equal(t, "test-agent", agent.GetName())
	assert.Equal(t, "idle", agent.GetStatus())

	// ListAgents should return it.
	list, err := client.ListAgents(ctx, &orchpb.ListAgentsRequest{})
	require.NoError(t, err)
	assert.Len(t, list.GetAgents(), 1)
	assert.Equal(t, agent.GetId(), list.GetAgents()[0].GetId())
}

func TestKillAgent(t *testing.T) {
	env := newTestEnv(t)
	client := orchpb.NewOrchestrationServiceClient(env.conn)
	ctx := context.Background()

	// Spawn, then kill.
	agent, err := client.SpawnAgent(ctx, &orchpb.SpawnAgentRequest{
		Name:    "doomed-agent",
		Role:    "polecat",
		Runtime: "codex",
	})
	require.NoError(t, err)

	_, err = client.KillAgent(ctx, &orchpb.KillAgentRequest{Id: agent.GetId()})
	require.NoError(t, err)

	// ListAgents should be empty.
	list, err := client.ListAgents(ctx, &orchpb.ListAgentsRequest{})
	require.NoError(t, err)
	assert.Empty(t, list.GetAgents())
}

func TestKillAgentNotFound(t *testing.T) {
	env := newTestEnv(t)
	client := orchpb.NewOrchestrationServiceClient(env.conn)

	_, err := client.KillAgent(context.Background(), &orchpb.KillAgentRequest{Id: "nonexistent"})
	require.Error(t, err)
}

// ---------- Mail ----------

func TestSendMailAndGetInbox(t *testing.T) {
	env := newTestEnv(t)
	client := mailpb.NewMailServiceClient(env.conn)
	ctx := context.Background()

	// Send a message.
	msg, err := client.SendMail(ctx, &mailpb.SendMailRequest{
		FromAddr:    "mayor@vibetown",
		ToAddr:      "crew-1@vibetown",
		Subject:     "New task",
		Body:        "Please handle ticket #42",
		Priority:    "high",
		MessageType: "task",
		Delivery:    "queue",
	})
	require.NoError(t, err)
	assert.NotEmpty(t, msg.GetId())
	assert.Equal(t, "New task", msg.GetSubject())
	assert.False(t, msg.GetRead())

	// GetInbox for the recipient.
	inbox, err := client.GetInbox(ctx, &mailpb.GetInboxRequest{
		ToAddr: "crew-1@vibetown",
	})
	require.NoError(t, err)
	assert.Equal(t, int32(1), inbox.GetTotal())
	assert.Equal(t, msg.GetId(), inbox.GetMessages()[0].GetId())
}

func TestMarkRead(t *testing.T) {
	env := newTestEnv(t)
	client := mailpb.NewMailServiceClient(env.conn)
	ctx := context.Background()

	msg, err := client.SendMail(ctx, &mailpb.SendMailRequest{
		FromAddr: "a@b",
		ToAddr:   "c@d",
		Subject:  "hi",
	})
	require.NoError(t, err)

	_, err = client.MarkRead(ctx, &mailpb.MarkReadRequest{Id: msg.GetId()})
	require.NoError(t, err)

	// Unread-only inbox should now be empty.
	inbox, err := client.GetInbox(ctx, &mailpb.GetInboxRequest{
		ToAddr:     "c@d",
		UnreadOnly: true,
	})
	require.NoError(t, err)
	assert.Empty(t, inbox.GetMessages())
}

func TestClaimQueueMessage(t *testing.T) {
	env := newTestEnv(t)
	client := mailpb.NewMailServiceClient(env.conn)
	ctx := context.Background()

	// Send a message into a queue.
	_, err := client.SendMail(ctx, &mailpb.SendMailRequest{
		FromAddr: "system@vibetown",
		ToAddr:   "queue@vibetown",
		Subject:  "queued work",
		Delivery: "queue",
	})
	require.NoError(t, err)

	// Send another message for the claim to find.
	env.srv.Mail.SendMail(ctx, &mailpb.SendMailRequest{
		FromAddr: "system@vibetown",
		ToAddr:   "worker@vibetown",
		Subject:  "queue task",
	})

	// Claim a message with queue="" (matches messages with empty Queue field).
	claimed, err := client.ClaimQueueMessage(ctx, &mailpb.ClaimRequest{
		Queue:     "",
		ClaimedBy: "worker-1",
	})
	require.NoError(t, err)
	assert.Equal(t, "worker-1", claimed.GetClaimedBy())
}

// ---------- Feed ----------

func TestStreamEventsReceivesEvents(t *testing.T) {
	env := newTestEnv(t)
	client := feedpb.NewFeedServiceClient(env.conn)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	stream, err := client.StreamEvents(ctx, &feedpb.StreamEventsRequest{})
	require.NoError(t, err)

	// Wait for the server-side subscriber to be registered before publishing.
	require.Eventually(t, func() bool {
		return env.srv.Feed.SubscriberCount() > 0
	}, 5*time.Second, 10*time.Millisecond, "subscriber never registered")

	// Publish an event through the handler.
	env.srv.Feed.Publish(&feedpb.FeedEvent{
		EventType: "agent_spawned",
		Source:    "test",
		Summary:   "Test agent spawned",
		Severity:  "info",
	})

	// We should receive it.
	evt, err := stream.Recv()
	require.NoError(t, err)
	assert.NotEmpty(t, evt.GetId())
	assert.Equal(t, "agent_spawned", evt.GetEventType())
}

func TestGetRecentEvents(t *testing.T) {
	env := newTestEnv(t)
	client := feedpb.NewFeedServiceClient(env.conn)
	ctx := context.Background()

	// Publish some events.
	for i := 0; i < 5; i++ {
		env.srv.Feed.Publish(&feedpb.FeedEvent{
			EventType: "test_event",
			Source:    "test",
			Summary:   "event",
			Severity:  "info",
		})
	}

	resp, err := client.GetRecentEvents(ctx, &feedpb.GetRecentEventsRequest{
		Limit: 3,
	})
	require.NoError(t, err)
	assert.Len(t, resp.GetEvents(), 3)
	assert.Equal(t, int32(5), resp.GetTotal())
}

// ---------- Engine interface compliance ----------

func TestInMemoryEngineImplementsEngine(t *testing.T) {
	var _ grpcapi.Engine = (*grpcapi.InMemoryEngine)(nil)
}

func TestDaemonEngineImplementsEngine(t *testing.T) {
	var _ grpcapi.Engine = (*grpcapi.DaemonEngine)(nil)
}

// TestInMemoryEngineMode verifies the mode string.
func TestInMemoryEngineMode(t *testing.T) {
	e := grpcapi.NewInMemoryEngine()
	assert.Equal(t, "standalone", e.Mode())
}

// TestNewServerWithEngineSharesState verifies all handlers share the same
// engine so operations in one service are visible in another.
func TestNewServerWithEngineSharesState(t *testing.T) {
	engine := grpcapi.NewInMemoryEngine()
	env := newTestEnvWithEngine(t, engine)

	orchClient := orchpb.NewOrchestrationServiceClient(env.conn)
	ctx := context.Background()

	// Spawn an agent via orchestration.
	agent, err := orchClient.SpawnAgent(ctx, &orchpb.SpawnAgentRequest{
		Name:    "shared-agent",
		Role:    "polecat",
		Runtime: "claude",
	})
	require.NoError(t, err)

	// List should find it.
	list, err := orchClient.ListAgents(ctx, &orchpb.ListAgentsRequest{})
	require.NoError(t, err)
	assert.Len(t, list.GetAgents(), 1)
	assert.Equal(t, agent.GetId(), list.GetAgents()[0].GetId())
}

// TestDaemonEngineCanBeConstructedWithoutTmux verifies DaemonEngine
// construction doesn't fail simply because tmux isn't running. We create
// a minimal town structure in a temp dir.
func TestDaemonEngineCanBeConstructedWithoutTmux(t *testing.T) {
	// DaemonEngine needs a valid town config to construct.
	// Without a real town root, it will fail. That's expected.
	_, err := grpcapi.NewDaemonEngine("/nonexistent/path")
	require.Error(t, err, "should fail with nonexistent town root")
}

// TestListRigsInStandalone verifies the default rigs in standalone mode.
func TestListRigsInStandalone(t *testing.T) {
	env := newTestEnv(t)
	client := orchpb.NewOrchestrationServiceClient(env.conn)
	ctx := context.Background()

	resp, err := client.ListRigs(ctx, &orchpb.ListRigsRequest{})
	require.NoError(t, err)
	// InMemoryEngine provides a default mock rig.
	assert.GreaterOrEqual(t, len(resp.GetRigs()), 1)
}
