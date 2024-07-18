function animateOut(element, duration) {
  let start = null;
  const startOpacity = 1;

  function step(timestamp) {
    if (!start) start = timestamp;
    const progress = (timestamp - start) / (duration * 1000);

    if (progress < 1) {
      const easeProgress = easeInOutCubic(progress);
      element.style.opacity = startOpacity - easeProgress;
      requestAnimationFrame(step);
    } else {
      element.style.opacity = 0;
    }
  }

  requestAnimationFrame(step);
}

function animateIn(element, duration) {
  let start = null;
  const startOpacity = 0;

  function step(timestamp) {
    if (!start) start = timestamp;
    const progress = (timestamp - start) / (duration * 1000);

    if (progress < 1) {
      const easeProgress = easeInOutCubic(progress);
      element.style.opacity = startOpacity + easeProgress;
      requestAnimationFrame(step);
    } else {
      element.style.opacity = 1;
    }
  }

  requestAnimationFrame(step);
}

function easeInOutCubic(t) {
  return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
}
