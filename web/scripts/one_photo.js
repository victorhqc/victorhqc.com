(function () {
  if (!window.__active_collection__) return;

  const ac = window.__active_collection__;

  registerEsc();

  function registerEsc() {
    document.addEventListener("keyup", (e) => {
      if (e.key === "Escape") {
        htmx.ajax("GET", ac.ajax_path, {
          source: `li[data-collection="${ac.name}"]`,
          target: ".portfolio__photos",
        });
      }
    });
  }
})();
