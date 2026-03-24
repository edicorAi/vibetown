package git_test

import (
	"github.com/edicorai/vibetown/engine/internal/beads"
	"github.com/edicorai/vibetown/engine/internal/git"
)

// Compile-time assertion: Git must satisfy BranchChecker.
var _ beads.BranchChecker = (*git.Git)(nil)
