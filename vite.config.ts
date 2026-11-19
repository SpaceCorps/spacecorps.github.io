import { resolve } from "node:path";
import { defineConfig } from "vite-plus";

// Static multi-page site. Everything else (blog, fonts, tokens, agent files) lives in public/
// and is copied to dist/ as is.
export default defineConfig({
  build: {
    outDir: "dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        about: resolve(__dirname, "about.html"),
        contact: resolve(__dirname, "contact.html"),
        privacy: resolve(__dirname, "privacy.html"),
        notfound: resolve(__dirname, "404.html"),
      },
    },
  },
});
