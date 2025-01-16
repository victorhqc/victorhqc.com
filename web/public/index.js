(function () {
  const stack = new PhotoStack({
    stackSelector: "#index__photos-stack",
    slideSelector: ".index__photo-slide",
    xAxisChange: __IS_MOBILE__ ? 8 : 20,
    yAxisChange: __IS_MOBILE__ ? 8 : 20,
    withKeyboardBindings: true,
  });

  stack.init();
})();
