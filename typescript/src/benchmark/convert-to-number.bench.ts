import { Bench } from "tinybench";
import { convertToNumber } from "../utils/convert-to-number";

async function main() {
    const bench = new Bench({ time: 100 });
    const successCases: {
        id: number;
        input: string;
    }[] = [
        {
            id: 0,
            input: "2",
        },
        {
            id: 1,
            input: "231",
        },
        {
            id: 2,
            input: "3129837",
        },
        {
            id: 3,
            input: "213n23kk3",
        },
        {
            id: 4,
            input: "123e213e",
        },
    ];

    successCases.map((value) => {
        bench.add(`id: ${value.id}`, () => {
            convertToNumber(value.input);
        });
    });

    await bench.warmup();
    await bench.run();

    console.log("Convert To Number result");
    console.table(bench.table());
}

main();
