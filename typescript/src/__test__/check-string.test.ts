import * as CompareString from "../utils/check-string";

const { isEmail, simpleCheckRole } = jest.requireActual<typeof CompareString>("../utils/check-string.ts");

interface SuccessCaseEmail {
    id: number;
    input: string;
    shouldBe: boolean;
}

const successCases: SuccessCaseEmail[] = [
    {
        id: 0,
        input: "admin@gmail.com",
        shouldBe: true,
    },
    {
        id: 1,
        input: "student@student.univ.ac.id",
        shouldBe: true,
    },
    {
        id: 2,
        input: "not a email",
        shouldBe: false,
    },
    {
        id: 3,
        input: "asdfg@asdf.asd",
        shouldBe: false,
    },
];

describe("Test Check is Email Fucntion", () => {
    it.each(successCases)("success case $id", ({ input, shouldBe }) => {
        const result = isEmail(input);
        expect(result).toEqual(shouldBe);
    });
});

const successCasesRole: {
    id: number;
    input: string;
    shouldBe: string;
}[] = [
    {
        id: 0,
        input: "U",
        shouldBe: "users",
    },
    {
        id: 1,
        input: "A",
        shouldBe: "admin",
    },
    {
        id: 2,
        input: "SA",
        shouldBe: "super admin",
    },
    {
        id: 3,
        input: "u",
        shouldBe: "unkown",
    },
    {
        id: 4,
        input: "MN",
        shouldBe: "unkown",
    },
];

describe("Test simple check role Function", () => {
    it.each(successCasesRole)("success case $id", ({ input, shouldBe }) => {
        const result = simpleCheckRole(input);
        expect(result).toBe(shouldBe);
    });
});
