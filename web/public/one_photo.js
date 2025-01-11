(function () {
  if (!window.__active_collection__ || !window.__open_photo__) return;

  document.title = `victorhqc.com - ${window.__open_photo__.photo.title}`;

  registerKeyboardNavigation();
  registerIconToggle();

  function registerIconToggle() {
    const icon = document.querySelector(".photo-info__icon");
    if (!icon) return;

    icon.addEventListener("click", toggleInfo);
  }

  function toggleInfo() {
    const icon = document.querySelector(".photo-info__icon");
    const target = document.querySelector(".open__photo .photo__container");
    const info = document.querySelector(".photo-info__wrapper");
    if (!target || !info || !icon) return;

    if (target.classList.contains("photo--hidden")) {
      info.classList.add("photo--hidden");
      target.classList.remove("photo--hidden");
      icon.classList.remove("photo-info__icon--black");
    } else {
      info.classList.remove("photo--hidden");
      target.classList.add("photo--hidden");
      icon.classList.add("photo-info__icon--black");
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
          target: ".portfolio__photos",
        })
        .then(() => {
          cleanupListeners(listeners);
        });
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
          target: ".portfolio__photos",
        })
        .then(() => {
          cleanupListeners(listeners);
        });
    };

    const goNext = () => {
      htmx
        .ajax("GET", nextPath, {
          source: ".next-photo-ref",
          target: ".portfolio__photos",
        })
        .then(() => {
          cleanupListeners(listeners);
        });
    };

    const registerArrows = () => {
      const left = document.querySelector(".photo-info__left-arrow");
      const right = document.querySelector(".photo-info__right-arrow");

      if (!left || !right) return;

      left.addEventListener("click", goPrev);
      right.addEventListener("click", goNext);
    };

    const listeners = [
      listenerEsc,
      listenerRightArrow,
      listenerLeftArrow,
      listenerSpace,
    ];

    registerArrows();
    addListeners(listeners);
  }

  function cleanupListeners(listeners) {
    for (const listener of listeners) {
      document.removeEventListener("keyup", listener);
    }
  }

  function addListeners(listeners) {
    for (const listener of listeners) {
      document.addEventListener("keyup", listener);
    }
  }
})();
