(async function () {
  const REGEX_TRANSFORM =
    /(translate3d\(([-+]?[0-9]+px,?\s?){3}\))\s?(rotate\([-+]?[0-9]+deg\))/gi;
  const REGEX_TRANSLATE3D =
    /translate3d\(((-?[0-9]+)px,?\s?)(((\+|-)?[0-9]+)px,?\s?)(((\+|-)?[0-9]+)px,?\s?)\)/gi;
  const Z_AXIS_CHANGE = 10;
  const Y_AXIS_CHANGE = 20;

  /**
   * @typedef {Object} ScrambledElement
   * @property {number} degrees
   * @property {number} x
   *
   * @typedef {"UP" | "DOWN"} Direction
   */

  const photos = document.querySelectorAll(".photo-slide");

  await waitForAllToLoad(photos);
  init();

  function init() {
    const scrambledPhotos = calculateScramblePositions(photos);

    for (const { photo, x, y, z, degrees } of scrambledPhotos) {
      photo.style.transform = `translate3d(0px, 0px, 0px) rotate(0deg)`;
      setTimeout(() => {
        photo.style.transform = `translate3d(${x}px, ${y}px, ${z}px) rotate(${degrees}deg)`;
      }, 50);
    }

    const stack = document.querySelector("#photos-stack");
    if (stack) {
      addWheelEvent(stack, ".photo-slide", scrambledPhotos);
    }
  }

  /**
   *
   * @param {HTMLElement} wrapper
   * @param {string} photosSelector
   * @param {Array<ScrambledElement>} scrambled
   */
  function addWheelEvent(wrapper, photosSelector, scrambled) {
    let isThrottled = false;
    wrapper.addEventListener("wheel", (event) => {
      event.preventDefault();

      if (isThrottled) return;
      const photos = document.querySelectorAll(photosSelector);
      const direction = event.deltaY > 0 ? "UP" : "DOWN";

      let sortedPhotos;
      /*
      When an animation starts, the `beforeRenderCb`, the position of the element that will be placed on top of
      the stack, needs to be resetted or the animation will look janky and out of place. This callback ensures
      that the position is in the appropriate place BEFORE the element is rendered back in the DOM.
      */
      switch (direction) {
        case "UP":
          sortedPhotos = sortPhotos(
            photos,
            (maxLength, index) => (index === 0 ? maxLength : index - 1),
            (element, index, last) => {
              if (index !== last) return;

              const { x, degrees } = findOriginalOrThrow(scrambled, element);
              const y = Y_AXIS_CHANGE * last;
              const z = Z_AXIS_CHANGE * last;

              element.style.transform = `translate3d(${x}px, ${y}px, -${z}px) rotate(${degrees}deg)`;
              element.style.zIndex = -2;
              element.style.transitionDuration = "150ms";
            },
          );
          break;
        case "DOWN":
          sortedPhotos = sortPhotos(
            photos,
            (maxLength, index) => (index === 0 ? maxLength : index - 1),
            (element, index, last) => {
              if (index !== last) return;
              const { x, degrees } = findOriginalOrThrow(scrambled, element);
              const y = Y_AXIS_CHANGE * last;
              const z = Z_AXIS_CHANGE * last;

              element.style.transform = `translate3d(${x}px, -${y}px, -${z}px) rotate(${degrees}deg)`;
              element.style.zIndex = -2;
              element.style.transitionDuration = "150ms";
            },
          );
          break;
      }

      setTimeout(() => {
        animateOnMovement(sortedPhotos, direction, scrambled);
      }, 50);

      isThrottled = true;
      setTimeout(() => (isThrottled = false), 600);
    });
  }

  /**
   *
   * @param {NodeListOf<HTMLElement>} elements
   * @param {Direction} direction
   * @param {Array<ScrambledElement>} scrambled
   */
  function animateOnMovement(elements, direction, scrambled) {
    const last = elements.length - 1;
    elements.forEach((element, index) => {
      const newZ = calculateZAxis(element, index, last, direction);
      const newY = calculateYAxis(element, index, last, direction);
      const { x, degrees } = findOriginalOrThrow(scrambled, element);

      // This means the photo is on top of the stack, so we want to restore the original "Y" value and reset the "Z" to 0
      if (index === last) {
        setTimeout(() => {
          element.style.zIndex = 0;
        }, 1);

        setTimeout(() => {
          element.style.transform = `translate3d(${x}px, 0px, 0px) rotate(${degrees}deg)`;
          element.style.transformOrigin = "";
          element.style.zIndex = 0;
        }, 50);
      } else {
        element.style.transitionDuration = "300ms";
        element.style.transform = `translate3d(${x}px, ${newY}px, ${newZ}px) rotate(${degrees}deg)`;
      }

      setTimeout(() => (element.style.transitionDuration = "0ms"), 300);
    });
  }

  /**
   * @callback sortPhotosCallback
   * @param {maxLength} number
   * @param {number} index
   * @returns {number}
   */

  /**
   * Executes the CB **Before** Rendering the element again in the document. Useful to apply a style on
   * mount.
   *
   * @callback beforeRenderCallback
   * @param {HTMLElement} element
   * @param {number} index
   * @param {number} last
   */

  /**
   *
   * @param {NodeListOf<Element>} elms
   * @param {sortPhotosCallback} sortCb
   * @param {beforeRenderCallback} beforeRenderCb
   */
  function sortPhotos(elms, sortCb, beforeRenderCb) {
    if (elms.length === 0) return Array.from(elms);

    const firstElement = elms[0];

    const parent = firstElement.parentNode;
    if (!parent) return Array.from(elms);

    const last = elms.length - 1;
    const elements = Array.from(elms)
      .map((element, index) => {
        const newIndex = sortCb(last, index);

        return { element, newIndex };
      })
      .sort((a, b) => a.newIndex - b.newIndex)
      .map(({ element }) => element);

    elements.forEach((element, index) => {
      element.remove();
      beforeRenderCb(element, index, last);

      parent.appendChild(element);
    });

    return elements;
  }

  /**
   *
   * @param {HTMLElement} el
   * @param {number} index
   * @param {number} last
   * @param {Direction} direction
   */
  function calculateZAxis(el, index, last, direction) {
    const { z } = getTransform(el);
    const isMain = index === last;

    if (isMain) {
      return 0;
    }

    if (z === 0) {
      return -Z_AXIS_CHANGE * (last - index);
    }

    return z - Z_AXIS_CHANGE;
  }

  /**
   *
   * @param {HTMLElement} el
   * @param {number} index
   * @param {number} last
   * @param {Direction} direction
   */
  function calculateYAxis(el, index, last, direction) {
    const { y } = getTransform(el);
    const isMain = index === last;

    if (isMain) {
      return 0;
    }

    if (direction === "DOWN") {
      if (y === 0) {
        return Y_AXIS_CHANGE * (last - index);
      }

      return y + Y_AXIS_CHANGE;
    }

    if (y === 0) {
      return -Y_AXIS_CHANGE * (last - index);
    }

    return y - Y_AXIS_CHANGE;
  }

  /**
   *
   * @param {number} index
   * @returns
   */
  function calculateDegrees(index) {
    const minDeg = index * 4 + 1;
    const maxDeg = index * 5 + 5;
    return randomIntFromInterval(minDeg, maxDeg);
  }

  /**
   *
   * @param {HTMLElement} element
   */
  function getTransform(element) {
    const originalTransform = element.style.transform;

    const matches = [...originalTransform.matchAll(REGEX_TRANSFORM)];
    if (matches.length === 0) {
      return { x: 0, y: 0, z: 0, rotate: `rotate(0deg)` };
    }

    const translate = matches[0][1];
    const rotate = matches[0][3];

    const translateMatches = [...translate.matchAll(REGEX_TRANSLATE3D)];
    const x = parseInt(translateMatches[0][2]);
    const y = parseInt(translateMatches[0][4]);
    const z = parseInt(translateMatches[0][7]);

    return { x, y, z, rotate };
  }

  /**
   * @param {Array<ScrambledElement} scrambled
   * @param {HTMLElement} element
   */
  function findOriginalOrThrow(scrambled, element) {
    const original = scrambled.find(({ photo }) => photo === element);
    if (!original) {
      throw new Error("Could not find original");
    }

    return original;
  }

  /**
   *
   * @param {NodeListOf<HTMLElement>} elements
   * @returns {Array.<{ photo: HTMLElement, x: number, y: number, z: number, degrees: number }>}
   */
  function calculateScramblePositions(elements) {
    const last = elements.length - 1;

    /** @type {Array.<{ photo: HTMLElement, x: number, y: number, z: number, degrees: number }>} */
    const scrambled = [];
    for (const [i, photo] of elements.entries()) {
      const y = calculateYAxis(photo, i, last, "DOWN");
      const z = calculateZAxis(photo, i, last, "DOWN");
      const { degrees, x } = calculateScramblePosition(i, last);
      scrambled.push({ photo, x, y, z, degrees });
    }

    return scrambled;
  }

  /**
   *
   * @param {number} index
   * @param {number} last
   * @returns {ScrambledElement}
   */
  function calculateScramblePosition(index, last) {
    if (index === last) {
      return { x: 0, degrees: 0 };
    }

    const degrees = calculateDegrees(index);

    const minX = randomPositiveNegative(20);
    const maxX = randomPositiveNegative(40);
    const x = randomIntFromInterval(minX, maxX);

    return { degrees, x };
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
   * @param {NodeListOf<HTMLElement>} photoSliders
   * @returns
   */
  function waitForAllToLoad(photoSliders) {
    /** @type {Array<HTMLElement>} */
    let loaded = [];
    return new Promise(async (resolve, reject) => {
      for (const slide of photoSliders) {
        const photo = slide.querySelector("img");

        photo.onload = (event) => {
          loaded.push(event.target);
        };
      }

      let index = 0;
      let totalWaitingTime = 0;
      while (true) {
        if (loaded.length === photos.length) {
          resolve();
          break;
        }

        if (index === 500) {
          reject(new Error("Could not load all photos"));
        }

        waitTime = Math.log(index + 1) * 50;
        totalWaitingTime += waitTime;
        await waitFor(waitTime);
        index++;
      }
    });
  }

  /**
   *
   * @param {number} ms
   * @returns {Promise<void>}
   */
  function waitFor(ms = 50) {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve();
      }, ms);
    });
  }
})();
