// Copies the Geist variable fonts (latin) from node_modules into public/fonts, so the site serves
// its own fonts: no Google Fonts request, nothing leaves the visitor's browser.
import { cpSync, mkdirSync } from "node:fs";
const out = "public/fonts";
mkdirSync(out, { recursive: true });
for (const [pkg, name] of [["geist", "geist"], ["geist-mono", "geist-mono"]]) {
  cpSync(`node_modules/@fontsource-variable/${pkg}/files/${name}-latin-wght-normal.woff2`, `${out}/${name}-latin.woff2`);
}
