(function () {
  const ACTIVE = "main_menu__element--active";

  const menu = document.querySelector(".main-menu");
  if (!menu) return;

  setInitialActive();

  function setInitialActive() {
    const routeMatches = [
      ...window.location.pathname.matchAll(/photography/gi),
    ];

    const query =
      routeMatches.length > 0
        ? `[data-mainmenu="${routeMatches[0][0]}"]`
        : `[data-mainmenu="home"]`;

    const active = menu.querySelector(query);
    if (!active) return;

    active.classList.add(ACTIVE);
  }
})();
