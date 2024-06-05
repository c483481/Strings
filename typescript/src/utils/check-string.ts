export function isEmail(str: string): boolean {
    return /[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+(?:[A-Z]{2}|com|org|net|gov|mil|biz|info|mobi|name|aero|jobs|museum|id)\b/.test(
        str
    );
}

export function simpleCheckRole(role: string): string {
    switch (role) {
        case "U":
            return "users";
        case "A":
            return "admin";
        case "SA":
            return "super admin";
        default:
            return "unkown";
    }
}
