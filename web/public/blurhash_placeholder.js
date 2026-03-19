(function () {
  "use strict";

  var bh = new BlurhashCanvas();

  function initBlurhashPlaceholders(root) {
    bh.init(root);

    // Hide the canvas once the real image finishes loading.
    var wrappers = (root || document).querySelectorAll("[data-blurhash]");
    wrappers.forEach(function (wrapper) {
      var img = wrapper.querySelector("img");
      var canvas = wrapper.querySelector("canvas");
      if (!img || !canvas) return;

      function hideCanvas() {
        canvas.style.display = "none";
      }

      if (img.complete) {
        hideCanvas();
      } else {
        img.addEventListener("load", hideCanvas);
      }
    });
  }

  // Initialize on page load.
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () {
      initBlurhashPlaceholders();
    });
  } else {
    initBlurhashPlaceholders();
  }

  // Re-initialize when HTMX swaps content.
  document.body.addEventListener("htmx:afterSwap", function (event) {
    initBlurhashPlaceholders(event.detail.target);
  });
})();
