(function () {
  const photos = document.querySelectorAll(".photo");
  const stack = document.querySelector("#photos-stack");

  scramblePhotos(photos);

  if (stack) {
    console.log("Enabling swipe to", stack);
    swipe(stack, photos);
  }

  /**
   *
   * @param {NodeListof<Element>} elements
   */
  function scramblePhotos(elements) {
    const size = elements.length;
    for (const [i, photo] of elements.entries()) {
      if (i === size - 1) continue;

      const minDeg = i * 4 + 1;
      const maxDeg = i * 5 + 5;
      const degrees = randomIntFromInterval(minDeg, maxDeg);

      const minX = randomPositiveNegative(30);
      const maxX = randomPositiveNegative(50);
      const x = randomIntFromInterval(minX, maxX);

      const minY = randomPositiveNegative(30);
      const maxY = randomPositiveNegative(50);
      const y = randomIntFromInterval(minY, maxY);

      displaceAndRotateElement(photo, x, y, degrees);
    }
  }

  /**
   *
   * @param {HTMLElement} el
   * @param {number} x
   * @param {number} y
   * @param {number} degrees
   */
  function displaceAndRotateElement(el, x, y, degrees) {
    console.log(`Displacing ${x},${y} and rotating ${degrees}deg`);
    el.style.transform = `translate(${x}px, ${y}px) rotate(${degrees}deg)`;
  }

  /**
   *
   * @param {number} min
   * @param {number} max
   * @returns
   */
  function randomIntFromInterval(min, max) {
    return Math.floor(Math.random() * (max - min + 1) + min);
  }

  /**
   *
   * @param {number} number
   * @returns
   */
  function randomPositiveNegative(number) {
    const sign = Math.random() < 0.5 ? -1 : 1;

    return number * sign;
  }

  /**
   *
   * @param {Element} wrapper
   * @param {NodeListOf<Element>} photos
   */
  function swipe(wrapper, photos) {
    addWheelEvent(wrapper, photos);

    wrapper.addEventListener("touchstart", (event) => {
      console.log("TOUCH START", event);
      // startX = event.touches[0].clientX;
    });

    wrapper.addEventListener("touchmove", (event) => {
      console.log("TOUCH MOVE", event);
    });
  }

  /**
   *
   * @param {Element} wrapper
   * @param {NodeListOf<Element>} photos
   */
  function addWheelEvent(wrapper, photos) {
    const photosLength = photos.length;
    console.log("PHOTOS LENGTH", photosLength);

    const activeIndex = getIndexFromActive(photos);
    console.log("ACTIVE INDEX", activeIndex);

    let isThrottled = false;
    wrapper.addEventListener("wheel", (event) => {
      if (isThrottled) return;
      console.log("WHEEL EVENT", event);

      const activeIndex = getIndexFromActive(photos);
      console.log("ACTIVE INDEX", activeIndex);
      if (activeIndex < 0) return;

      const direction = event.deltaY > 0 ? "UP" : "DOWN";
      console.log("DIRECTION ", direction);

      let nextIndexActive = -1;

      switch (direction) {
        case "UP":
          nextIndexActive =
            activeIndex === 0 ? photosLength - 1 : activeIndex - 1;
          break;
        case "DOWN":
          nextIndexActive =
            activeIndex === photosLength - 1 ? 0 : activeIndex + 1;
          break;
      }

      console.log("NEW INDEX", nextIndexActive);

      for (const [index, photo] of photos.entries()) {
        if (index === activeIndex) {
          photo.classList.remove("active");
        }

        if (index === nextIndexActive) {
          photo.classList.add("active");
        }
      }

      console.log("--");

      isThrottled = true;
      setTimeout(() => (isThrottled = false), 800);
    });
  }

  /**
   *
   * @param {NodeListOf<Element>} photos
   */
  function getIndexFromActive(photos) {
    let activeIndex = -1;
    for (const [index, photo] of photos.entries()) {
      if (photo.classList.contains("active")) {
        activeIndex = index;
        break;
      }
    }

    return activeIndex;
  }
})();
