import type { Config } from "jest";

const config: Config = {
  automock: true,
  collectCoverage: true,
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
    },
  },
  moduleFileExtensions: ["ts", "js"],
  coverageProvider: "babel",
  preset: "ts-jest",
};

export default config;
