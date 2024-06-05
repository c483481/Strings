export function replaceString(str: string, delimeter: string, combine: string): string {
    return str.split(delimeter).join(combine);
}
