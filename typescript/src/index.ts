import { convertToNumber } from "./utils/convert-to-number";
import { compareString, safeCompareString } from "./utils/compare-string";
import { isEmail } from "./utils/check-string";

console.log(`result convert from "42": ${convertToNumber("42")}`);
console.log(`result convert from "abcd": ${convertToNumber("abcd")}`);
console.log(`compare result : ${compareString("testing", "testing")}`);
console.log(`compare result : ${safeCompareString("testing", "testing")}`);
console.log(`is email admin@gmail.com: ${isEmail("admin@gmail.com")}`);
console.log(`is email asdf@adsf.adsf: ${isEmail("asdf@adsf.adsf")}`);
console.log(`is email asdf@univ.ac.id: ${isEmail("asdf@univ.ac.id")}`);
