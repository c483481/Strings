import { Bench } from "tinybench";
import { replaceString } from "../utils/replace-strings";

async function main() {
    const bench = new Bench({ time: 100 });
    const successCases: {
        id: number;
        input: string;
    }[] = [
        {
            id: 0,
            input: "The Quick Brown Fox",
        },
        {
            id: 1,
            input: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
        },
    ];

    successCases.map((value) => {
        bench.add(`id: ${value.id}`, () => {
            replaceString(value.input, " ", "-");
        });
    });

    await bench.warmup();
    await bench.run();

    console.log("Replace String Result");
    console.table(bench.table());
}

main();
