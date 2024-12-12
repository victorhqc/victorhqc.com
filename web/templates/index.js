(function () {
  /**
   * @typedef {Array.<{photo: Element, x: number, y: number, z: number, degrees: number}>} ScrambledElement
   * @typedef {"UP" | "DOWN"} Direction
   */

  const stack = document.querySelector("#photos-stack");
  const photos = document.querySelectorAll(".photo");

  const scrambledPhotos = calculateScramblePositions(photos);

  for (const { photo, x, y, z, degrees } of scrambledPhotos) {
    displaceAndRotateElement(photo, 0, 0, 0, 0);
    setTimeout(() => {
      displaceAndRotateElement(photo, x, y, z, degrees);
    }, 50);
  }

  let originalTransforms;

  if (stack) {
    swipe(scrambledPhotos, stack, ".photo");
  }

  /**
   *
   * @param {NodeListOf<Element>} elements
   */
  function calculateScramblePositions(elements) {
    const last = elements.length - 1;

    const scrambled = [];
    for (const [i, photo] of elements.entries()) {
      if (i === last) {
        scrambled.push({ photo, x: 0, y: 0, z: 0, degrees: 0 });

        continue;
      }

      const { x, y, z, degrees } = calculateScramblePosition(i, last);
      scrambled.push({ photo, x, y, z, degrees });
    }

    return scrambled;
  }

  /**
   *
   * @param {number} index
   * @param {number} last
   * @returns {Array<ScrambledElement>}
   */
  function calculateScramblePosition(index, last) {
    const minDeg = index * 4 + 1;
    const maxDeg = index * 5 + 5;
    const degrees = randomIntFromInterval(minDeg, maxDeg);

    const minX = randomPositiveNegative(30);
    const maxX = randomPositiveNegative(50);
    const x = randomIntFromInterval(minX, maxX);

    const minY = randomPositiveNegative(30);
    const maxY = randomPositiveNegative(50);
    const y = randomIntFromInterval(minY, maxY);

    const z = calculateZAxis(index, last);

    return { x, y, z, degrees };
  }

  /**
   *
   * @param {number} index
   * @param {number} last
   * @returns
   */
  function calculateZAxis(index, last) {
    return index === last ? 0 : -8 * (last - index);
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
    el.style.transform = `translate3d(${x}px, ${y}px, ${z}px) rotate(${degrees}deg)`;
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
   * @param {Array.<ScrambledElement>} scrambledPhotos
   * @param {Element} wrapper
   * @param {string} photosSelector
   */
  function swipe(scrambledPhotos, wrapper, photosSelector) {
    addWheelEvent(wrapper, photosSelector, scrambledPhotos);

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
   * @param {Array.<ScrambledElement>} scrambledPhotos
   */
  function addWheelEvent(wrapper, photosSelector, scrambledPhotos) {
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
        animateOnMovement(sortedPhotos, direction, scrambledPhotos);
      }, 50);

      isThrottled = true;
      setTimeout(() => (isThrottled = false), 800);
    });
  }

  /**
   *
   * @param {NodeListOf<Element>} elements
   * @param {Direction} direction
   * @param {Array.<ScrambledElement>} scrambledPhotos
   */
  function animateOnMovement(elements, direction, scrambledPhotos) {
    console.log("DIRECTION", direction);
    const last = elements.length - 1;
    elements.forEach((element, index) => {
      const original = scrambledPhotos.find(({ photo }) => photo === element);

      if (!original) {
        throw new Error("Could not find originals");
      }

      const newZ = calculateZAxis(index, last);

      const { x, y, degrees } = original;

      // This means the photo is on top of the stack, so we want to restore the original "Y" value and reset the "Z" to 0
      if (index === last) {
        element.style.transform = `translate3d(${x}px, ${y}px, 0px) rotate(${degrees}deg)`;
        return;
      }

      element.style.transform = `translate3d(${x}px, ${y}px, ${newZ}px) rotate(${degrees}deg)`;
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
