import { convertToNumber } from "./utils/convert-to-number";
import { compareString, safeCompareString } from "./utils/compare-string";

console.log(`result convert from "42": ${convertToNumber("42")}`);
console.log(`result convert from "abcd": ${convertToNumber("abcd")}`);
console.log(`compare result : ${compareString("testing", "testing")}`);
console.log(`compare result : ${safeCompareString("testing", "testing")}`);
