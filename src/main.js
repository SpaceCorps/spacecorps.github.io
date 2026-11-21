import "./styles/site.css";
import "./wave.js";

if (document.getElementById("status")) import("./status.js");

const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;

/* ---- Theme: system by default, manual override remembered (the inline head script applies it first). ---- */
const root = document.documentElement;
for (const b of document.querySelectorAll(".theme-btn")) {
  b.addEventListener("click", () => {
    const dark = !root.classList.contains("dark");
    root.classList.toggle("dark", dark);
    try { localStorage.setItem("sc-theme", dark ? "dark" : "light"); } catch {}
    b.setAttribute("aria-pressed", String(dark));
  });
  b.setAttribute("aria-pressed", String(root.classList.contains("dark")));
}

/* ---- Mobile menu: close on link click or Escape ---- */
const menu = document.querySelector("details.menu");
if (menu) {
  menu.addEventListener("click", (e) => { if (e.target.closest("a")) menu.open = false; });
  addEventListener("keydown", (e) => { if (e.key === "Escape" && menu.open) { menu.open = false; menu.querySelector("summary").focus(); } });
}

/* ---- CLI directory: search + category filter (the full list is in the HTML without JS) ---- */
const dir = document.getElementById("dir");
if (dir) {
  const items = [...dir.children];
  const q = document.getElementById("q");
  const count = document.getElementById("count");
  const empty = document.getElementById("empty");
  const btns = [...document.querySelectorAll(".filters button")];
  let cat = "all";
  const apply = () => {
    const term = q.value.trim().toLowerCase();
    let n = 0;
    for (const li of items) {
      const ok = (cat === "all" || li.dataset.cat === cat) && (!term || li.dataset.s.includes(term));
      li.hidden = !ok;
      if (ok) n++;
    }
    count.textContent = n === items.length ? `${n} command-line tools` : `${n} of ${items.length} command-line tools`;
    empty.hidden = n !== 0;
  };
  q.addEventListener("input", apply);
  for (const b of btns) b.addEventListener("click", () => {
    cat = b.dataset.cat;
    for (const o of btns) o.setAttribute("aria-pressed", String(o === b));
    apply();
  });
  addEventListener("keydown", (e) => {
    if (e.key === "/" && !/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "") && !e.metaKey && !e.ctrlKey) {
      e.preventDefault();
      q.focus();
      q.scrollIntoView({ block: "center" });
    }
  });
}

/* ---- Starfield: ~140 points on one canvas. Static under reduced motion, paused off-screen/hidden. ---- */
const cv = document.getElementById("stars");
const initStars = () => {
  if (!cv) return;
  const ctx = cv.getContext("2d");
  const dpr = Math.min(devicePixelRatio || 1, 1.5);
  let w = 0, h = 0, raf = 0, visible = true, px = 0, py = 0, tx = 0, ty = 0;
  const stars = Array.from({ length: innerWidth < 700 ? 70 : 140 }, () => ({ x: Math.random(), y: Math.random(), z: Math.random(), t: Math.random() * 6.28 }));
  const size = () => {
    const r = cv.getBoundingClientRect();
    w = r.width; h = r.height;
    cv.width = Math.round(w * dpr); cv.height = Math.round(h * dpr);
  };
  const draw = (time) => {
    px += (tx - px) * 0.05; py += (ty - py) * 0.05;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, w, h);
    for (const s of stars) {
      const depth = 0.25 + s.z * 0.75;
      const x = ((s.x * w + px * 26 * depth) % w + w) % w;
      const y = ((s.y * h + py * 18 * depth + (reduce ? 0 : time * 0.002 * depth)) % h + h) % h;
      const tw = reduce ? 1 : 0.65 + 0.35 * Math.sin(time * 0.0012 + s.t);
      ctx.globalAlpha = (0.25 + 0.65 * depth) * tw;
      ctx.fillStyle = s.z > 0.82 ? "#9ef5da" : "#ffffff";
      const r = 0.4 + depth * 1.1;
      ctx.fillRect(x, y, r, r);
    }
    if (!reduce && visible) raf = requestAnimationFrame(draw);
  };
  const start = () => { cancelAnimationFrame(raf); raf = requestAnimationFrame(draw); };
  size();
  draw(0);
  if (!reduce) {
    new IntersectionObserver(([e]) => { visible = e.isIntersecting; if (visible && !document.hidden) start(); }).observe(cv);
    document.addEventListener("visibilitychange", () => { if (!document.hidden && visible) start(); else cancelAnimationFrame(raf); });
    cv.parentElement.addEventListener("pointermove", (e) => {
      const r = cv.getBoundingClientRect();
      tx = (e.clientX - r.left) / r.width - 0.5; ty = (e.clientY - r.top) / r.height - 0.5;
    }, { passive: true });
    start();
  }
  new ResizeObserver(() => { size(); draw(performance.now()); }).observe(cv);
};
(window.requestIdleCallback || ((f) => setTimeout(f, 200)))(initStars, { timeout: 1500 });
