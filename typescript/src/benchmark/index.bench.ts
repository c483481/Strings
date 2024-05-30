import glob from "glob";
import path from "path";

// // Find all test files matching the pattern
const testFiles = glob.sync(path.join(__dirname, "*.bench.ts"));

testFiles.forEach(async (file: string) => {
    await import(file);
});
