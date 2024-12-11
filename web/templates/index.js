(function () {
  const stack = document.querySelector("#photos-stack");
  const photos = document.querySelectorAll(".photo");

  scramblePhotos(photos);

  if (stack) {
    swipe(stack, ".photo");
  }

  /**
   *
   * @param {NodeListof<Element>} elements
   */
  function scramblePhotos(elements) {
    const length = elements.length;
    for (const [i, photo] of elements.entries()) {
      if (i === length - 1) continue;

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
   * @param {string} photosSelector
   */
  function swipe(wrapper, photosSelector) {
    addWheelEvent(wrapper, photosSelector);

    wrapper.addEventListener("touchstart", (event) => {
      console.log("TOUCH START", event);
    });

    wrapper.addEventListener("touchmove", (event) => {
      console.log("TOUCH MOVE", event);
    });
  }

  /**
   *
   * @param {Element} wrapper
   * @param {string} photosSelector
   */
  function addWheelEvent(wrapper, photosSelector) {
    let isThrottled = false;
    wrapper.addEventListener("wheel", (event) => {
      if (isThrottled) return;
      const photos = document.querySelectorAll(photosSelector);

      const direction = event.deltaY > 0 ? "UP" : "DOWN";

      switch (direction) {
        case "UP":
          sortPhotos(photos, (maxLength, index) =>
            index === 0 ? maxLength : index - 1,
          );
          break;
        case "DOWN":
          sortPhotos(photos, (maxLength, index) =>
            index === maxLength ? 0 : index + 1,
          );
          break;
      }

      isThrottled = true;
      setTimeout(() => (isThrottled = false), 800);
    });
  }

  /**
   * @callback sortPhotosCallback
   * @param {maxLength} number
   * @param {number} index
   * @returns {number}
   */

  /**
   *
   * @param {NodeListOf<Element>} elms
   * @param {sortPhotosCallback} cb
   */
  function sortPhotos(elms, cb) {
    if (elms.length === 0) return;

    const firstElement = elms[0];

    const parent = firstElement.parentNode;
    if (!parent) return;

    const maxLength = elms.length - 1;
    const elements = Array.from(elms)
      .map((element, index) => {
        const newIndex = cb(maxLength, index);

        return { element, newIndex };
      })
      .sort((a, b) => a.newIndex - b.newIndex)
      .map(({ element }) => element);

    elements.forEach((element) => {
      element.remove();
      parent.appendChild(element);
    });
  }
})();
