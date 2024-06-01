import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/client/views/**/*.{tsx,ts}",
    "./src/server/views/**/*.rs",
  ],
};

export default config;
