(function () {
  if (!window.__active_collection__) return;

  registerEsc();

  function registerEsc() {
    document.addEventListener("keyup", (e) => {
      if (e.key === "Escape") {
        window.location.href = `${location.origin}${window.__active_collection__.path}`;
      }
    });
  }
})();
