package polecat

import (
	"os"
	"testing"

	"github.com/edicorai/vibetown/engine/internal/testutil"
)

func TestMain(m *testing.M) {
	code := m.Run()
	testutil.TerminateDoltContainer()
	os.Exit(code)
}
