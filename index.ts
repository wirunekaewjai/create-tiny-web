#!/usr/bin/env bun
import { cp, readFile, writeFile } from "node:fs/promises";
import path from "node:path";

const src = path.join(__dirname, "template");
const dst = process.argv[2];

if (!dst) {
  throw "Please provide destination";
}

const projectName = path.dirname(path.join(dst, "test.ts")).split("/").pop()?.toLowerCase();

if (!projectName) {
  throw "Unable to generate project name";
}

await cp(src, dst, {
  force: true,
  preserveTimestamps: true,
  recursive: true,
});

const files = [
  "Cargo.toml",
  "package.json",
];

for (const file of files) {
  const filePath = path.join(dst, file);

  const text = await readFile(filePath, "utf8");
  const textOut = text.replaceAll("PROJECT_NAME", projectName);

  await writeFile(filePath, textOut, "utf8");
}

console.log(`project ${projectName} created`);