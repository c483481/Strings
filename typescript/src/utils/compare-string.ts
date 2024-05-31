export function compareString(a: unknown, b: unknown): boolean {
    if (!a || !b || typeof a !== "string" || typeof b !== "string") {
        return false;
    }

    const lenA = a.length;
    if (lenA !== b.length) {
        return false;
    }

    for (let i = 0; i < lenA; i++) {
        if (!(a.charCodeAt(i) === b.charCodeAt(i))) {
            return false;
        }
    }

    return true;
}

export function safeCompareString(a: unknown, b: unknown): boolean {
    if (!a || !b || typeof a !== "string" || typeof b !== "string") {
        return false;
    }

    const lenA = a.length;
    if (lenA !== b.length) {
        return false;
    }

    let result: number = 0;

    for (let i = 0; i < lenA; i++) {
        result |= a.charCodeAt(i) ^ b.charCodeAt(i);
    }

    return result === 0;
}
