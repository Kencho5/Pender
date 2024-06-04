var currentImg = 0;
function slideImage() {
  const images = document.querySelectorAll(".image");

  if (currentImg >= images.length - 1) currentImg = 0;

  for (const image of images) {
    if (image.classList.contains("active-image")) {
      currentImg++;

      const nextImg = document.querySelector(`[image-id="${currentImg}"]`);
      image.classList.remove("active-image");
      nextImg.classList.add("active-image");

      break;
    }
  }
}
