/** @type {import("prettier").Config} */
const config = {
    trailingComma: "all",
    tabWidth: 4,
    semi: true,
    singleQuote: false,
    plugins: ["prettier-plugin-jinja-template", "prettier-plugin-tailwindcss"],
};

export default config;
