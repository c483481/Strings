import { Bench } from "tinybench";
import { compareString, safeCompareString } from "../utils/compare-string";

async function main() {
    const bench = new Bench({ time: 100 });
    const successCases: {
        str1: string;
        str2: string;
    }[] = [
        {
            str1: "daodjwaiojdaidjaiwo",
            str2: "daodjwaiojdaidjaiwo",
        },
        {
            str1: "ytadwbuhjkasrtvbuhdjkmdasdaysbuidasndawyduin0",
            str2: "duaywheq2uh1ihe198edqwj,djsw8ajidawjiodjasjdo",
        },
        {
            str1: "21eqw56saw,dhue20e189eklh821709ekjz82q0epkj2eu8e2kjaq298e1ejki8aewk",
            str2: "asdnasjdnasjkdnasjkndajndjakndwjndqindjwndasdasd9j29ekaojdaoijdawdiaowdkanmdlad",
        },
        {
            str1: "2193",
            str2: "2193",
        },
        {
            str1: "sdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdai",
            str2: "sdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdai",
        },
        {
            str1: "sdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdaisdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdaisdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdai",
            str2: "sdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdaisdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdaisdasdjaiodjasidjasidjaoisdjasiodjaisdjasodjaisjdai",
        },
    ];

    successCases.map(({ str1, str2 }, index) => {
        bench.add(`compare string id: ${index + 1}`, () => {
            compareString(str1, str2);
        });
    });

    successCases.map(({ str1, str2 }, index) => {
        bench.add(`safe compare string id: ${index + 1}`, () => {
            safeCompareString(str1, str2);
        });
    });

    await bench.warmup();
    await bench.run();

    console.log("Compare string result");
    console.table(bench.table());
}

main();
