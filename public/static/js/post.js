var images = document.querySelectorAll(".image");
var dots = document.querySelectorAll(".dot");
var currentImageIndex = 0;

function showImage(index) {
  images[currentImageIndex].classList.remove("active-image");
  dots[currentImageIndex].classList.remove("active-dot");

  currentImageIndex = index;

  images[currentImageIndex].classList.add("active-image");
  dots[currentImageIndex].classList.add("active-dot");
}

function showPreviousImage() {
  let newIndex = (currentImageIndex - 1 + images.length) % images.length;
  showImage(newIndex);
}

function showNextImage() {
  let newIndex = (currentImageIndex + 1) % images.length;
  showImage(newIndex);
}
