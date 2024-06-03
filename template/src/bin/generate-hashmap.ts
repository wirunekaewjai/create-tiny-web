import { Glob } from "bun";

const minify = process.argv.includes("--minify");

const assetsGlob = new Glob("assets/**/*").scanSync({
  onlyFiles: true,
});

const publicGlob = new Glob("public/**/*").scanSync({
  onlyFiles: true,
});

const map: Record<string, string> = {};

for (const file of assetsGlob) {
  const data = await Bun.file(file).arrayBuffer();
  const hash = Bun.hash.wyhash(data).toString(16);

  map["/" + file] = hash;
}

for (const file of publicGlob) {
  const data = await Bun.file(file).arrayBuffer();
  const hash = Bun.hash.wyhash(data).toString(16);

  map["/" + file.slice(7)] = hash;
}

const text = minify ? JSON.stringify(map) : JSON.stringify(map, null, 2);
Bun.write(Bun.file(".cache/hashmap.json"), text);