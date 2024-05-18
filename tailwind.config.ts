import type { Config } from "tailwindcss";

export default {
    content: ["./templates/**/*.html.jinja"],
    plugins: [],
    theme: {
        extend: {},
    },
} satisfies Config;
