var images = document.querySelectorAll(".image");
var currentImageIndex = 0;

function showImage(index) {
  images[currentImageIndex].classList.remove("active-image");
  images[currentImageIndex].style.display = "none";
  currentImageIndex = index;
  images[currentImageIndex].classList.add("active-image");
  images[currentImageIndex].style.display = "block";
}

function showPreviousImage() {
  let newIndex = (currentImageIndex - 1 + images.length) % images.length;
  showImage(newIndex);
}

function showNextImage() {
  let newIndex = (currentImageIndex + 1) % images.length;
  showImage(newIndex);
}
