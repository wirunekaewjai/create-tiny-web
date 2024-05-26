import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/client/components/**/*.{tsx,ts}",
    "./src/client/pages/**/*.{tsx,ts}",
    "./src/client/views/**/*.{tsx,ts}",
    "./src/server/views/**/*.rs",
  ],
};

export default config;
