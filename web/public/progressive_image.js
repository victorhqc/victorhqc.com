(function () {
  "use strict";

  function loadProgressiveImage(img) {
    const hdSrc = img.getAttribute("data-src-hd");

    if (!hdSrc) return;

    // Create a new image to preload the HD version
    const hdImage = new Image();

    hdImage.onload = function () {
      // Swap to HD image
      img.src = hdSrc;

      // Remove data attribute as it's no longer needed
      img.removeAttribute("data-src-hd");
    };

    hdImage.onerror = function () {
      console.warn("Failed to load HD image:", hdSrc);
    };

    // Start loading the HD image
    hdImage.src = hdSrc;

    // Check if image loaded from cache (synchronously)
    if (hdImage.complete) {
      img.src = hdSrc;
      img.removeAttribute("data-src-hd");
    }
  }

  function initProgressiveImages() {
    const progressiveImages = document.querySelectorAll(
      ".progressive-image[data-src-hd]",
    );

    progressiveImages.forEach(function (img) {
      if (img.complete) {
        // If low-res image is already loaded, start loading HD version
        loadProgressiveImage(img);
      } else {
        // Wait for low-res image to load first
        img.addEventListener("load", function () {
          loadProgressiveImage(img);
        });
      }
    });
  }

  // Initialize on page load
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initProgressiveImages);
  } else {
    initProgressiveImages();
  }

  // Re-initialize when HTMX swaps content
  document.body.addEventListener("htmx:afterSwap", function (event) {
    initProgressiveImages();
  });
})();
