import * as CompareString from "../utils/compare-string";

const { compareString, safeCompareString } = jest.requireActual<typeof CompareString>("../utils/compare-string.ts");

interface SuccessCaseCompareString {
    id: number;
    input: {
        str1: unknown;
        str2: unknown;
    };
    shouldBe: boolean;
}

const successCases: SuccessCaseCompareString[] = [
    {
        id: 1,
        input: {
            str1: "daodjwaiojdaidjaiwo",
            str2: "daodjwaiojdaidjaiwo",
        },
        shouldBe: true,
    },
    {
        id: 2,
        input: {
            str1: "ytadwbuhjkasrtvbuhdjkmdasdaysbuidasndawyduin0",
            str2: "duaywheq2uh1ihe198edqwj,djsw8ajidawjiodjasjdo",
        },
        shouldBe: false,
    },
    {
        id: 3,
        input: {
            str1: "asdnasjdnasjkdnasjkndajndjakndwjndqindjwndasdasd9j29ekaojdaoijdawdiaowdkanmdlad",
            str2: "21eqw56saw,dhue20e189eklh821709ekjz82q0epkj2eu8e2kjaq298e1ejki8aewk",
        },
        shouldBe: false,
    },
    {
        id: 4,
        input: {
            str1: "sdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdai",
            str2: "sdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdai",
        },
        shouldBe: true,
    },
    {
        id: 5,
        input: {
            str1: "",
            str2: "asdsa",
        },
        shouldBe: false,
    },
    {
        id: 6,
        input: {
            str1: undefined,
            str2: 213,
        },
        shouldBe: false,
    },
];

describe("Test Compare String Fucntion", () => {
    it.each(successCases)("success case $id", ({ input, shouldBe }) => {
        const result = compareString(input.str1, input.str2);
        expect(result).toEqual(shouldBe);
    });
});

describe("Test Safe Compare String Fucntion", () => {
    it.each(successCases)("success case $id", ({ input, shouldBe }) => {
        const result = safeCompareString(input.str1, input.str2);
        expect(result).toEqual(shouldBe);
    });
});
