package _test

import (
	"github.com/stretchr/testify/require"
	"str/utils"
	"testing"
)

func TestCheckIsEmail(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		shouldBe bool
	}{
		{
			name:     "_test 1",
			input:    "admin@gmail.com",
			shouldBe: true,
		},
		{
			name:     "_test 2",
			input:    "student@student.univ.ac.id",
			shouldBe: true,
		},
		{
			name:     "_test 3",
			input:    "not a email",
			shouldBe: false,
		},
		{
			name:     "_test 4",
			input:    "asdfg@asdf.asd",
			shouldBe: false,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result := utils.IsEmail(test.input)
			require.Equal(t, test.shouldBe, result)
		})
	}
}

func BenchmarkCheckIsEmail(b *testing.B) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "_test 1",
			input: "admin@gmail.com",
		},
		{
			name:  "_test 2",
			input: "student@student.univ.ac.id",
		},
		{
			name:  "_test 3",
			input: "not a email",
		},
		{
			name:  "_test 4",
			input: "asdfg@asdf.asd",
		},
	}

	for _, test := range tests {
		b.Run(test.name, func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				utils.IsEmail(test.input)
			}
		})
	}
}
