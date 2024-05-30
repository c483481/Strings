package _test

import (
	"github.com/stretchr/testify/require"
	"str/utils"
	"testing"
)

func TestConvertToNumber(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		shouldBe int
	}{
		{
			name:     "_test 1",
			input:    "2",
			shouldBe: 2,
		},
		{
			name:     "_test 2",
			input:    "231",
			shouldBe: 231,
		},
		{
			name:     "_test 3",
			input:    "3129837",
			shouldBe: 3129837,
		},
		{
			name:     "_test 4",
			input:    "213n23kk3",
			shouldBe: 0,
		},
		{
			name:     "_test 5",
			input:    "123e213e",
			shouldBe: 0,
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result := utils.ConvertToNumber(test.input)
			require.Equal(t, test.shouldBe, result)
		})
	}
}

func BenchmarkConvertToNumber(b *testing.B) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "_test 1",
			input: "2",
		},
		{
			name:  "_test 2",
			input: "231",
		},
		{
			name:  "_test 3",
			input: "3129837",
		},
		{
			name:  "_test 4",
			input: "213n23kk3",
		},
		{
			name:  "_test 5",
			input: "123e213e",
		},
	}

	for _, test := range tests {
		b.Run(test.name, func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				utils.ConvertToNumber(test.input)
			}
		})
	}
}
