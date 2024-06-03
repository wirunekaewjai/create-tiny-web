#!/usr/bin/env bun
import { cp, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";

const src = path.join(__dirname, "template");
const dst = process.argv[2];
const nameOrCommand = process.argv[3];

if (!dst) {
  throw "Please provide destination";
}

if (!nameOrCommand) {
  throw "Please provide project name or command";
}

if (nameOrCommand === "--sync-jetpack") {
  const jetpackSrc = path.join(src, "jetpack");
  const jetpackDst = path.join(dst, "jetpack");

  await rm(jetpackDst, {
    force: true,
    recursive: true,
  });

  await cp(jetpackSrc, jetpackDst, {
    force: true,
    preserveTimestamps: true,
    recursive: true,
  });
}

else {
  await cp(src, dst, {
    force: true,
    preserveTimestamps: true,
    recursive: true,
  });

  const files = [
    "Cargo.toml",
    "Dockerfile",
    "package.json",
  ];

  for (const file of files) {
    const filePath = path.join(dst, file);

    const text = await readFile(filePath, "utf8");
    const textOut = text.replaceAll("PROJECT_NAME", nameOrCommand);

    await writeFile(filePath, textOut, "utf8");
  }

  console.log(`project ${nameOrCommand} created`);
}
