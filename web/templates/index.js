const photos = document.querySelectorAll(".photo");
const size = photos.length;

for (const [i, photo] of photos.entries()) {
  if (i === size - 1) continue;

  const minDeg = i * 5 + 1;
  const maxDeg = i * 5 + 5;
  const degrees = randomIntFromInterval(minDeg, maxDeg);

  const isEven = i % 2 === 0;

  const minX = randomPositiveNegative(30);
  const maxX = randomPositiveNegative(50);
  const x = randomIntFromInterval(minX, maxX);

  const minY = randomPositiveNegative(30);
  const maxY = randomPositiveNegative(50);
  const y = randomIntFromInterval(minY, maxY);

  displaceAndRotateElement(photo, x, y, degrees);
}

function displaceAndRotateElement(el, x, y, degrees) {
  console.log(`Displacing ${x},${y} and rotating ${degrees}deg`);
  el.style.transform = `translate(${x}px, ${y}px) rotate(${degrees}deg)`;
}

function randomIntFromInterval(min, max) {
  return Math.floor(Math.random() * (max - min + 1) + min);
}

function randomPositiveNegative(number) {
  const sign = Math.random() < 0.5 ? -1 : 1;

  return number * sign;
}
