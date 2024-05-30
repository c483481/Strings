export function convertToNumber(str: string): number {
    const result = Number(str);

    if (isNaN(result)) {
        return 0;
    }

    return result;
}
