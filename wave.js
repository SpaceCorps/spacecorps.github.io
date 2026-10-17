/* Click wave: a click sends a ripple outward that briefly colours the text it passes.
   Pure Web Animations (no layout changes). Skipped for prefers-reduced-motion. */
(function () {
  if (matchMedia("(prefers-reduced-motion: reduce)").matches) return;
  var SEL = "h1,h2,h3,h4,p,li,a,td,th,figcaption,.stat-val,.stat-lbl,.section-title,.section-subtitle";
  var RADIUS = 520, SPEED = 0.9, DUR = 700; // px, px per ms, ms
  var COLORS = ["#38bdf8", "#818cf8", "#f472b6", "#fbbf24", "#34d399"];
  var n = 0;
  function ownText(el) {
    for (var c = el.firstChild; c; c = c.nextSibling) if (c.nodeType === 3 && c.nodeValue.trim()) return true;
    return false;
  }
  addEventListener("pointerdown", function (e) {
    if (e.button !== 0) return;
    var tag = e.target && e.target.tagName;
    if (tag === "VIDEO" || tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;
    var color = COLORS[n++ % COLORS.length];
    var els = document.querySelectorAll(SEL), limit = 90;
    for (var i = 0; i < els.length && limit > 0; i++) {
      var el = els[i];
      if (!ownText(el)) continue;
      var r = el.getBoundingClientRect();
      if (r.bottom < 0 || r.top > innerHeight || r.width === 0) continue;
      var dx = Math.max(r.left - e.clientX, 0, e.clientX - r.right);
      var dy = Math.max(r.top - e.clientY, 0, e.clientY - r.bottom);
      var d = Math.hypot(dx, dy);
      if (d > RADIUS) continue;
      limit--;
      var base = getComputedStyle(el).color;
      el.animate([{ color: base }, { color: color, offset: 0.3 }, { color: base }],
        { duration: DUR, delay: d / SPEED, easing: "ease-out" });
    }
  }, { passive: true });
})();
