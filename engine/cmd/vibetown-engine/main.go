package main

import (
	"flag"
	"log"
	"os"
	"os/signal"
	"strconv"
	"syscall"

	"github.com/edicorai/vibetown/engine/internal/grpcapi"
)

func main() {
	defaultPort := 50051
	if p := os.Getenv("VIBETOWN_PORT"); p != "" {
		if v, err := strconv.Atoi(p); err == nil {
			defaultPort = v
		}
	}

	port := flag.Int("port", defaultPort, "gRPC listen port")
	mode := flag.String("mode", "standalone", "engine mode: standalone (in-memory) or daemon (real Gastown)")
	townRoot := flag.String("town-root", os.Getenv("GT_TOWN_ROOT"), "Gas Town workspace root (required for daemon mode)")
	flag.Parse()

	var srv *grpcapi.GRPCServer

	switch *mode {
	case "standalone":
		log.Println("vibetown-engine: starting in standalone mode (in-memory)")
		srv = grpcapi.NewServer(*port)

	case "daemon":
		if *townRoot == "" {
			log.Fatal("vibetown-engine: --town-root (or GT_TOWN_ROOT env) is required for daemon mode")
		}
		log.Printf("vibetown-engine: starting in daemon mode (town-root=%s)", *townRoot)

		engine, err := grpcapi.NewDaemonEngine(*townRoot)
		if err != nil {
			log.Fatalf("vibetown-engine: failed to initialize daemon engine: %v", err)
		}

		// Start the feed curator for real event processing.
		if err := engine.StartCurator(); err != nil {
			log.Printf("vibetown-engine: warning: feed curator failed to start: %v", err)
		}

		srv = grpcapi.NewServerWithEngine(*port, engine)

		// Ensure curator is stopped on shutdown.
		defer engine.StopCurator()

	default:
		log.Fatalf("vibetown-engine: unknown mode %q (use 'standalone' or 'daemon')", *mode)
	}

	// Handle graceful shutdown.
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		sig := <-sigCh
		log.Printf("vibetown-engine: received %s, shutting down", sig)
		srv.Stop()
	}()

	log.Printf("vibetown-engine: starting on port %d", *port)
	if err := srv.Start(); err != nil {
		log.Fatalf("vibetown-engine: %v", err)
	}
}
