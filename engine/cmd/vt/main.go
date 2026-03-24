// gt is the Gas Town CLI for managing multi-agent workspaces.
package main

import (
	"os"

	"github.com/edicorai/vibetown/engine/internal/cmd"
)

func main() {
	os.Exit(cmd.Execute())
}
