package tree_sitter_mumps_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-mumps"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_mumps.Language())
	if language == nil {
		t.Errorf("Error loading Mumps grammar")
	}
}
