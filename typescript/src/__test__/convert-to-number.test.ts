import * as ConvertToNumber from "../utils/convert-to-number";

const { convertToNumber } = jest.requireActual<typeof ConvertToNumber>("../utils/convert-to-number.ts");

interface SuccessCaseConvertToNumber {
    id: number;
    input: string;
    shouldBe: number;
}

const successCases: SuccessCaseConvertToNumber[] = [
    {
        id: 0,
        input: "2",
        shouldBe: 2,
    },
    {
        id: 1,
        input: "231",
        shouldBe: 231,
    },
    {
        id: 2,
        input: "3129837",
        shouldBe: 3129837,
    },
    {
        id: 3,
        input: "213n23kk3",
        shouldBe: 0,
    },
    {
        id: 4,
        input: "123e213e",
        shouldBe: 0,
    },
];

describe("Test Convert To Number  Fucntion", () => {
    it.each(successCases)("success case $id", ({ input, shouldBe }) => {
        const result = convertToNumber(input);
        expect(result).toEqual(shouldBe);
    });
});
