import { resolve } from "node:path";
import { defineConfig } from "vite-plus";

// Static multi-page site. Everything else (blog, fonts, tokens, agent files) lives in public/
// and is copied to dist/ as is.
// The status page reads /api/status/*, which the production Caddy proxies to the status service (status/).
// In development and preview the same path goes to a locally running one (STATUS_ADDR, default 127.0.0.1:8090).
const status = { "/api/status": process.env.STATUS_DEV_URL ?? "http://127.0.0.1:8090" };

export default defineConfig({
  server: { proxy: status },
  preview: { proxy: status },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        about: resolve(__dirname, "about.html"),
        contact: resolve(__dirname, "contact.html"),
        privacy: resolve(__dirname, "privacy.html"),
        status: resolve(__dirname, "status.html"),
        notfound: resolve(__dirname, "404.html"),
        blog: resolve(__dirname, "blog/index.html"),
        blogSpacecorps: resolve(__dirname, "blog/spacecorps-2027.html"),
        blogSpace3d: resolve(__dirname, "blog/space3d-engine.html"),
        blogObop: resolve(__dirname, "blog/obop.html"),
        blogSpaceemit: resolve(__dirname, "blog/getelec-spaceemit.html"),
      },
    },
  },
});
