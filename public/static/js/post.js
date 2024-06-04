var currentImg = 0;
function changeImage(dir) {
  const images = document.querySelectorAll(".image");

  if (dir == "left" && currentImg == 0) currentImg = images.length;
  else if (dir == "right" && currentImg >= images.length - 1) currentImg = -1;

  for (const image of images) {
    if (image.classList.contains("active-image")) {
      if (dir == "left") currentImg--;
      else currentImg++;

      const nextImg = document.querySelector(`[image-id="${currentImg}"]`);
      image.classList.remove("active-image");
      nextImg.classList.add("active-image");

      break;
    }
  }
}
