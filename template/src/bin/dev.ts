import { $ } from "bun";
import { rm } from "node:fs/promises";

await rm(".cache", {
  force: true,
  recursive: true,
});

await rm("assets", {
  force: true,
  recursive: true,
});

const css = "tailwindcss -i ./tailwind.css -o ./assets/style.css";
const ts = "bun build src/client/app.ts --outdir assets --splitting --chunk-naming chunk-[hash].[ext]";
const map = "bun src/bin/generate-hashmap.ts";
const rs = "cargo run";

await $`cargo watch -s '${css} && ${ts} && ${map} && ${rs}'`;