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

await $`tailwindcss -i ./tailwind.css -o ./assets/style.css --minify`;
await $`bun build src/client/app.ts --outdir assets --minify --splitting`;
await $`bun src/bin/generate-hashmap.ts --minify`;