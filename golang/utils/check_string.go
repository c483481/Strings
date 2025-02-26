package utils

import (
	"regexp"
)

var re = regexp.MustCompile(`^[a-z0-9!#$%&'*+/=?^_` + "`" + `{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_` + "`" + `{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+(?:[A-Z]{2}|com|org|net|gov|mil|biz|info|mobi|name|aero|jobs|museum|id)\b$`)

func IsEmail(str string) bool {
	return re.MatchString(str)
}

func SimpleCheckRole(role string) string {
	switch role {
	case "U":
		return "users"
	case "A":
		return "admin"
	case "SA":
		return "super admin"
	default:
		return "unknown"
	}
}
