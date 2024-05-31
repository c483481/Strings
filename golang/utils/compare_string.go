package utils

func CompareString(a, b string) bool {
	if len(a) != len(b) {
		return false
	}

	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			return false
		}
	}

	return true
}

func SafeCompareString(a, b string) bool {
	if len(a) != len(b) {
		return false
	}
	lenA := len(a)
	var result byte = 0
	for i := 0; i < lenA; i++ {
		result |= a[i] ^ b[i]
	}

	return result == 0
}
