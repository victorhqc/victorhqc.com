(function () {
  const ACTIVE = "portfolio__collection-item--active";

  const menu = document.querySelector(".portfolio__collections-menu");
  if (!menu) return;

  const elements = menu.querySelectorAll("li");

  elements.forEach((e) => {
    e.addEventListener("click", onClick);
  });

  /**
   *
   * @param {MouseEvent} event
   */
  function onClick(event) {
    const active = document.querySelector(`.${ACTIVE}`);

    if (active) {
      active.classList.remove(ACTIVE);
    }

    event.target.classList.add(ACTIVE);
  }
})();
