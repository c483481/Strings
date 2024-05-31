package _test

import (
	"github.com/stretchr/testify/require"
	"str/utils"
	"testing"
)

func TestCompareString(t *testing.T) {
	tests := []struct {
		name     string
		str1     string
		str2     string
		shouldBe bool
	}{
		{
			name:     "_test 1",
			str1:     "same string",
			str2:     "same string",
			shouldBe: true,
		},
		{
			name:     "_test 2",
			str1:     "r2131230ck",
			str2:     "r1233129lk",
			shouldBe: false,
		},
		{
			name:     "_test 3",
			str1:     "very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string",
			str2:     "very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string",
			shouldBe: true,
		},
		{
			name:     "_test 4",
			str1:     "very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short",
			str2:     "very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_v",
			shouldBe: false,
		},
		{
			name:     "_test 5",
			str1:     "long string",
			str2:     "short string",
			shouldBe: false,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result := utils.CompareString(test.str1, test.str2)
			require.Equal(t, test.shouldBe, result)
		})
	}
}

func BenchmarkCompareString(b *testing.B) {
	tests := []struct {
		name string
		str1 string
		str2 string
	}{
		{
			name: "_test 1",
			str1: "same string",
			str2: "same string",
		},
		{
			name: "_test 2",
			str1: "r2131230ck",
			str2: "r1233129lk",
		},
		{
			name: "_test 3",
			str1: "very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string",
			str2: "very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string",
		},
		{
			name: "_test 4",
			str1: "very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short",
			str2: "very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_v",
		},
	}

	for _, test := range tests {
		b.Run(test.name, func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				utils.CompareString(test.str1, test.str2)
			}
		})
	}
}
