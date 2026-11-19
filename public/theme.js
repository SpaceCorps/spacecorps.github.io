/* Follows the visitor's colour scheme: the design system's tokens switch on the `.dark` class. */
(function () {
  var d = false;
  try { d = matchMedia("(prefers-color-scheme: dark)").matches; } catch (e) {}
  document.documentElement.classList.toggle("dark", d);
})();
