(function () {
  if (!window.__active_collection__ || !window.__open_photo__) return;

  document.title = `victorhqc.com - ${window.__open_photo__.photo.title}`;
  CLEANUP_EXISTING_LISTENERS();

  registerKeyboardNavigation();
  registerIconToggle();

  function registerIconToggle() {
    const icon = document.querySelector(".open-photo__icon");
    if (!icon) return;

    icon.addEventListener("click", toggleInfo);
  }

  /**
   * Toggles the information of the photo
   */
  function toggleInfo() {
    const icon = document.querySelector(".open-photo__icon");
    const info = document.querySelector(".open-photo__info-wrapper");

    if (!info || !icon) return;

    if (info.classList.contains("photo--hidden")) {
      console.log("Should show it");
      info.classList.remove("photo--hidden");
      icon.classList.add("open-photo__icon--black");
    } else {
      info.classList.add("photo--hidden");
      icon.classList.remove("open-photo__icon--black");
    }
  }

  function registerKeyboardNavigation() {
    const ac = window.__active_collection__;
    const data = window.__open_photo__;

    const nextPath = `/one_photo/${ac.name}/${data.next_id}`;
    const prevPath = `/one_photo/${ac.name}/${data.prev_id}`;

    const listenerSpace = (e) => {
      if (e.key !== " ") return;

      toggleInfo();
    };

    const listenerEsc = (e) => {
      if (e.key !== "Escape") return;

      htmx
        .ajax("GET", ac.ajax_path, {
          source: `li[data-collection="${ac.name}"]`,
          target: ".portfolio__photos-section",
        })
        .then(CLEANUP_EXISTING_LISTENERS);
    };

    const listenerRightArrow = (e) => {
      if (e.key !== "ArrowRight") return;

      goNext();
    };

    const listenerLeftArrow = (e) => {
      if (e.key !== "ArrowLeft") return;

      goPrev();
    };

    const goPrev = () => {
      htmx
        .ajax("GET", prevPath, {
          source: ".prev-photo-ref",
          target: ".portfolio__photos-section",
        })
        .then(CLEANUP_EXISTING_LISTENERS);
    };

    const goNext = () => {
      htmx
        .ajax("GET", nextPath, {
          source: ".next-photo-ref",
          target: ".portfolio__photos-section",
        })
        .then(CLEANUP_EXISTING_LISTENERS);
    };

    const listeners = [
      [listenerEsc, "keyup", document],
      [listenerRightArrow, "keyup", document],
      [listenerLeftArrow, "keyup", document],
      [listenerSpace, "keyup", document],
    ];

    addListeners(listeners);
  }

  function addListeners(listeners) {
    listeners.forEach(REGISTER_LISTENER);
  }
})();
