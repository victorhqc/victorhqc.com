(function () {
  const ACTIVE = "portfolio__collection-item--active";

  const menu = document.querySelector(".portfolio__collections-menu");
  if (!menu) return;

  setInitialActive();
  initMenu();

  function setInitialActive() {
    const routeMatches = [
      ...window.location.pathname.matchAll(/\/portfolio\/([a-z]+)/gi),
    ];

    const collection =
      routeMatches.length > 0 ? routeMatches[0][1] : "portfolio";

    const active = menu.querySelector(`[data-collection="${collection}"]`);
    if (!active) return;

    active.classList.add(ACTIVE);
  }

  function initMenu() {
    const elements = menu.querySelectorAll("li");

    elements.forEach((e) => {
      e.addEventListener("click", onClick);
    });
  }

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
