(function () {
  const stack = new PhotoStack({
    stackSelector: "#photos-stack",
    slideSelector: ".photo-slide",
    xAxisChange: __IS_MOBILE__ ? 8 : 20,
    yAxisChange: __IS_MOBILE__ ? 8 : 20,
  });

  stack.init();
})();
