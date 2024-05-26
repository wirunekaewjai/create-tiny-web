import { $ } from "bun";
import { styleText } from "node:util";

async function buildCss() {
  console.log(styleText("blue", "===== build css ====="));
  await $`tailwindcss -i ./tailwind.css -o ./assets/style.css --minify`;
  console.log();
}

async function buildScript() {
  console.log(styleText("blue", "===== build client script ====="));
  console.time("usage");

  const build = await Bun.build({
    entrypoints: [
      "src/client/app.ts",
    ],
    outdir: "assets",
    minify: true,
  });

  if (!build.success) {
    console.log(build.logs);
  }

  console.timeEnd("usage");
  console.log();
}

await buildCss();
await buildScript();