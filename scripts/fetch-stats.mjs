// Pulls real public numbers from the GitHub API (via the `gh` CLI) into data/stats.json.
// The site is static: run `pnpm run stats` when you want fresh numbers, review the diff, commit.
// Nothing here is estimated; a value that cannot be fetched is left out, never guessed.
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const gh = (path) => JSON.parse(execFileSync("gh", ["api", "--paginate", "--slurp", path], { encoding: "utf8", maxBuffer: 64e6 })).flat();
const ghOne = (path) => { try { return JSON.parse(execFileSync("gh", ["api", path], { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] })); } catch { return null; } };

const repos = gh("orgs/SpaceCorps/repos?per_page=100&type=public").filter((r) => !r.archived && !r.fork);
const cliRepos = JSON.parse(readFileSync("data/clis.json", "utf8")).map((c) => c.repo);
const cli = {};
for (const name of cliRepos) {
  const r = repos.find((x) => x.name === name);
  const rel = ghOne(`repos/SpaceCorps/${name}/releases/latest`);
  cli[name] = { pushed: r?.pushed_at ?? null, license: r?.license?.spdx_id ?? null, release: rel?.tag_name ?? null, released: rel?.published_at ?? null };
}
const releases = (name) => gh(`repos/SpaceCorps/${name}/releases?per_page=100`).filter((r) => !r.draft && !r.prerelease);
const play = releases("play");
const storybook = ghOne("repos/SpaceCorps/storybook/releases/latest");
const stats = {
  generatedAt: new Date().toISOString(),
  org: {
    publicRepos: repos.length,
    rustRepos: repos.filter((r) => r.language === "Rust").length,
    openLicensed: repos.filter((r) => ["MIT", "Apache-2.0"].includes(r.license?.spdx_id)).length,
  },
  game: { releases: play.length, latest: play[0]?.tag_name ?? null, latestDate: play[0]?.published_at ?? null, first: play.at(-1)?.published_at ?? null },
  designSystem: { latest: storybook?.tag_name ?? null },
  cli,
};
mkdirSync("data", { recursive: true });
writeFileSync("data/stats.json", JSON.stringify(stats, null, 2) + "\n");
console.log(JSON.stringify(stats.org), stats.game);
