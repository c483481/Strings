import { Bench } from "tinybench";
import { isEmail, simpleCheckRole } from "../utils/check-string";

async function main() {
    const bench = new Bench({ time: 100 });
    const successCases: string[] = ["admin@gmail.com", "student@student.univ.ac.id", "not a email", "asdfg@asdf.asd"];
    const successCasesRole: string[] = ["U", "A", "SA", "MA", "u"];

    successCases.map((value, index) => {
        bench.add(`check email id: ${index + 1}`, () => {
            isEmail(value);
        });
    });

    successCasesRole.map((value, index) => {
        bench.add(`check simple role id: ${index + 1}`, () => {
            simpleCheckRole(value);
        });
    });

    await bench.warmup();
    await bench.run();

    console.log("Check is Email Result");
    console.table(bench.table());
}

main();
