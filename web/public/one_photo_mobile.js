(function () {
  if (!window.__active_collection__ || !window.__open_photo__) return;

  document.title = `victorhqc.com - ${window.__open_photo__.photo.title}`;

  registerTouchHandling();
  function registerTouchHandling() {
    var wrapper = document.querySelector(".open-photo");
    if (!wrapper) return;

    var startX = 0;
    var startY = 0;
    var distX = 0;
    var distY = 0;
    var threshold = 50; // minimum distance for swipe
    var restraint = 100; // maximum distance perpendicular to swipe direction
    var allowedTime = 300; // maximum time allowed to travel that distance
    var startTime = 0;

    wrapper.addEventListener(
      "touchstart",
      function touchStart(e) {
        var touchobj = e.changedTouches[0];
        startX = touchobj.pageX;
        startY = touchobj.pageY;
        startTime = new Date().getTime();
      },
      { passive: true },
    );

    wrapper.addEventListener(
      "touchend",
      function touchEnd(e) {
        var touchobj = e.changedTouches[0];
        distX = touchobj.pageX - startX;
        distY = touchobj.pageY - startY;
        var elapsedTime = new Date().getTime() - startTime;

        if (elapsedTime <= allowedTime) {
          // Swipe left - next photo
          if (distX <= -threshold && Math.abs(distY) <= restraint) {
            e.preventDefault();
            var nextBtn = document.querySelector(".one-photo__right-arrow");
            if (nextBtn) nextBtn.click();
          }
          // Swipe right - previous photo
          else if (distX >= threshold && Math.abs(distY) <= restraint) {
            e.preventDefault();
            var prevBtn = document.querySelector(".one-photo__left-arrow");
            if (prevBtn) prevBtn.click();
          }
        }
      },
      { passive: false },
    );
  }
})();
