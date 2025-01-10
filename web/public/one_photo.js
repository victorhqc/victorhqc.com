(function () {
  if (!window.__active_collection__ || !window.__open_photo__) return;

  document.title = `victorhqc.com - ${window.__open_photo__.photo.title}`;

  registerKeyboardNavigation();
  registerIconToggle();
  adjustInfoHeight();

  function adjustInfoHeight() {
    const source = document.querySelector(".open__photo");
    const img = source.querySelector("img");
    const target = document.querySelector(".photo-info__wrapper");
    const icon = document.querySelector(".photo-info__icon");

    if (!target || !source || !icon || !img) {
      return;
    }

    img.onload = () => {
      const dimensions = source.getBoundingClientRect();
      target.style.height = `${dimensions.height}px`;
      icon.classList.remove("hidden");
    };
  }

  function registerIconToggle() {
    const icon = document.querySelector(".photo-info__icon");
    if (!icon) return;

    icon.addEventListener("click", () => {
      const target = document.querySelector(".open__photo");
      const info = document.querySelector(".photo-info__wrapper");
      if (!target || !info) return;

      if (target.classList.contains("photo--hidden")) {
        target.classList.remove("photo--hidden");
        icon.classList.remove("photo-info__icon--black");
      } else {
        info.classList.remove("invisible");
        target.classList.add("photo--hidden");
        icon.classList.add("photo-info__icon--black");
      }
    });
  }

  function registerKeyboardNavigation() {
    const ac = window.__active_collection__;
    const data = window.__open_photo__;

    const nextPath = `/one_photo/${ac.name}/${data.next_id}`;
    const prevPath = `/one_photo/${ac.name}/${data.prev_id}`;

    const listenerEsc = (e) => {
      if (e.key !== "Escape") return;

      htmx
        .ajax("GET", ac.ajax_path, {
          source: `li[data-collection="${ac.name}"]`,
          target: ".portfolio__photos",
        })
        .then(() => {
          document.removeEventListener("keyup", listenerEsc);
          document.removeEventListener("keyup", listenerRightArrow);
          document.removeEventListener("keyup", listenerLeftArrow);
        });
    };

    const listenerRightArrow = (e) => {
      if (e.key !== "ArrowRight") return;

      htmx
        .ajax("GET", nextPath, {
          source: ".next-photo-ref",
          target: ".portfolio__photos",
        })
        .then(() => {
          document.removeEventListener("keyup", listenerEsc);
          document.removeEventListener("keyup", listenerRightArrow);
          document.removeEventListener("keyup", listenerLeftArrow);
        });
    };

    const listenerLeftArrow = (e) => {
      if (e.key !== "ArrowLeft") return;

      htmx
        .ajax("GET", prevPath, {
          source: ".prev-photo-ref",
          target: ".portfolio__photos",
        })
        .then(() => {
          document.removeEventListener("keyup", listenerEsc);
          document.removeEventListener("keyup", listenerRightArrow);
          document.removeEventListener("keyup", listenerLeftArrow);
        });
    };

    document.addEventListener("keyup", listenerEsc);
    document.addEventListener("keyup", listenerRightArrow);
    document.addEventListener("keyup", listenerLeftArrow);
  }
})();
