import { sequentialWatch } from "@wirunekaewjai/ts/sequential-watch";
import { $, type Subprocess } from "bun";
import { styleText } from "node:util";

async function buildCss() {
  console.log(styleText("blue", "===== build css ====="));
  await $`tailwindcss -i ./tailwind.css -o ./assets/style.css`;
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
    minify: false,
  });

  if (!build.success) {
    console.log(build.logs);
  }

  console.timeEnd("usage");
  console.log();
}

let server: Subprocess | null = null;

await sequentialWatch([
  {
    dirs: [
      // define you watch directories

      //
      "src/client/views",
      "src/server/views",
    ],
    callback: buildCss,
  },
  {
    dirs: ["src/client"],
    callback: buildScript,
  },
  {
    dirs: ["src/server"],
    callback: () => {
      console.log(styleText("blue", "===== start server ====="));

      server?.kill();
      server = Bun.spawn({
        cmd: ["cargo", "run"],
        stdout: "inherit",
        stderr: "inherit",
      });
    },
  },
]);
