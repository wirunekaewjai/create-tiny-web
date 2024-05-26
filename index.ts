#!/usr/bin/env bun
import { cp } from "node:fs/promises";
import path from "node:path";

const src = path.join(__dirname, "template");
const dst = process.argv[2];

if (!dst) {
  throw "Please provide destination";
}

await cp(src, dst, {
  force: true,
  preserveTimestamps: true,
  recursive: true,
});