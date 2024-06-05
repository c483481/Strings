package _test

import (
	"github.com/stretchr/testify/require"
	"strings"
	"testing"
)

func TestReplaceString(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		shouldBe string
	}{
		{
			name:     "_test 1",
			input:    "The Quick Brown Fox",
			shouldBe: "The-Quick-Brown-Fox",
		},
		{
			name:     "_test 2",
			input:    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
			shouldBe: "Lorem-ipsum-dolor-sit-amet,-consectetur-adipiscing-elit.-Sed-do-eiusmod-tempor-incididunt-ut-labore-et-dolore-magna-aliqua.-Ut-enim-ad-minim-veniam,-quis-nostrud-exercitation-ullamco-laboris-nisi-ut-aliquip-ex-ea-commodo-consequat.-Duis-aute-irure-dolor-in-reprehenderit-in-voluptate-velit-esse-cillum-dolore-eu-fugiat-nulla-pariatur.",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result := strings.ReplaceAll(test.input, " ", "-")
			require.Equal(t, test.shouldBe, result)
		})
	}
}

func BenchmarkReplaceString(b *testing.B) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "_test 1",
			input: "The Quick Brown Fox",
		},
		{
			name:  "_test 2",
			input: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
		},
	}

	for _, test := range tests {
		b.Run(test.name, func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				strings.ReplaceAll(test.input, " ", "-")
			}
		})
	}
}
