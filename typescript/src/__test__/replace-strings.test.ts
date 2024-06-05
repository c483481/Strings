import * as ReplaceString from "../utils/replace-strings";

const { replaceString } = jest.requireActual<typeof ReplaceString>("../utils/replace-strings.ts");

const successCases: {
    id: number;
    input: string;
    shouldBe: string;
}[] = [
    {
        id: 0,
        input: "The Quick Brown Fox",
        shouldBe: "The-Quick-Brown-Fox",
    },
    {
        id: 1,
        input: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
        shouldBe:
            "Lorem-ipsum-dolor-sit-amet,-consectetur-adipiscing-elit.-Sed-do-eiusmod-tempor-incididunt-ut-labore-et-dolore-magna-aliqua.-Ut-enim-ad-minim-veniam,-quis-nostrud-exercitation-ullamco-laboris-nisi-ut-aliquip-ex-ea-commodo-consequat.-Duis-aute-irure-dolor-in-reprehenderit-in-voluptate-velit-esse-cillum-dolore-eu-fugiat-nulla-pariatur.",
    },
];

describe("Test all string Function", () => {
    it.each(successCases)("success case $id", ({ input, shouldBe }) => {
        const result = replaceString(input, " ", "-");
        expect(result).toBe(shouldBe);
    });
});
