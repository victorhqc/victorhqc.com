(function () {
  const stack = document.querySelector("#photos-stack");
  const photos = document.querySelectorAll(".photo");
  const REGEX_TRANSFORM =
    /(translate3d\(([-+]?[0-9]+px,?\s?){3}\))\s?(rotate\([-+]?[0-9]+deg\))/gi;
  const REGEX_TRANSLATE3D =
    /translate3d\(((-?[0-9]+)px,?\s?)(((\+|-)?[0-9]+)px,?\s?)(((\+|-)?[0-9]+)px,?\s?)\)/gi;

  scrambleElements(photos);

  if (stack) {
    swipe(stack, ".photo");
  }

  /**
   *
   * @param {NodeListOf<Element>} elements
   */
  function scrambleElements(elements) {
    const last = elements.length - 1;
    for (const [i, photo] of elements.entries()) {
      if (i === last) continue;

      displaceAndRotateElement(photo, 0, 0, 0, 0);
      setTimeout(() => {
        scrambleElement(i, photo);
      }, 50);
    }
  }

  /**
   *
   * @param {number} index
   * @param {Element} element
   */
  function scrambleElement(index, element) {
    const minDeg = index * 4 + 1;
    const maxDeg = index * 5 + 5;
    const degrees = randomIntFromInterval(minDeg, maxDeg);

    const minX = randomPositiveNegative(30);
    const maxX = randomPositiveNegative(50);
    const x = randomIntFromInterval(minX, maxX);

    const minY = randomPositiveNegative(30);
    const maxY = randomPositiveNegative(50);
    const y = randomIntFromInterval(minY, maxY);

    const z = -15 * (index + 1);

    displaceAndRotateElement(element, x, y, z, degrees);
  }

  /**
   *
   * @param {Element} el
   * @param {number} x
   * @param {number} y
   * @param {number} z
   * @param {number} degrees
   */
  function displaceAndRotateElement(el, x, y, z, degrees) {
    el.style.transform = `translate3d(${x}px, ${y}px, 0) rotate(${degrees}deg)`;
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

      let sortedPhotos;
      switch (direction) {
        case "UP":
          sortedPhotos = sortPhotos(photos, (maxLength, index) =>
            index === 0 ? maxLength : index - 1,
          );
          break;
        case "DOWN":
          sortedPhotos = sortPhotos(photos, (maxLength, index) =>
            index === maxLength ? 0 : index + 1,
          );
          break;
      }

      setTimeout(() => {
        animateOnMovement(sortedPhotos, direction);
      }, 50);

      isThrottled = true;
      setTimeout(() => (isThrottled = false), 500);
    });
  }

  /**
   * @typedef {"UP" | "DOWN"} Direction
   */

  /**
   *
   * @param {NodeListOf<Element>} elements
   * @param {Direction} direction
   */
  function animateOnMovement(elements, direction) {
    const last = elements.length - 1;
    elements.forEach((element, index) => {
      const originalTransform = element.style.transform;
      // element.style.transform = "";
      //
      const newZ = index === last ? 0 : -100 * (last - index);
      console.log("AMOUNT TO PUSH BACK", element, newZ);

      if (!originalTransform) {
        element.style.transform = `translate3d(0, 0, ${newZ}px) rotate(0deg)`;

        return;
      }

      const matches = [...originalTransform.matchAll(REGEX_TRANSFORM)];
      const translate = matches[0][1];
      const rotate = matches[0][3];
      console.log("STYLE TRANSFORM", translate, rotate);

      const translateMatches = [...translate.matchAll(REGEX_TRANSLATE3D)];
      const x = translateMatches[0][2];
      const y = translateMatches[0][4];
      const z = translateMatches[0][7];

      console.log("TRANSLATE MATCHES", x, y, z);

      element.style.transform = `translate3d(${x}px, ${y}px, ${newZ}px) ${rotate}`;

      // scrambleElement(index, element);
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
    if (elms.length === 0) return Array.from(elms);

    const firstElement = elms[0];

    const parent = firstElement.parentNode;
    if (!parent) return Array.from(elms);

    const maxLength = elms.length - 1;
    const elements = Array.from(elms)
      .map((element, index) => {
        const newIndex = cb(maxLength, index);

        return { element, newIndex };
      })
      .sort((a, b) => a.newIndex - b.newIndex)
      .map(({ element }) => element);

    elements.forEach((element, index) => {
      element.remove();
      parent.appendChild(element);
    });

    return elements;
  }
})();
