(function () {
  if (!window.__active_collection__) return;

  registerEsc();

  function registerEsc() {
    const ac = window.__active_collection__;

    const listener = (e) => {
      if (e.key === "Escape") {
        htmx
          .ajax("GET", ac.ajax_path, {
            source: `li[data-collection="${ac.name}"]`,
            target: ".portfolio__photos",
          })
          .then(() => {
            document.removeEventListener("keyup", listener);
          });
      }
    };

    document.addEventListener("keyup", listener);
  }
})();
