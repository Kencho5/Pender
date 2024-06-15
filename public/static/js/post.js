var images = document.querySelectorAll(".image");
var dots = document.querySelectorAll(".dot");
var overlay = document.getElementById("overlay");
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

var touchstartX = 0;
var touchendX = 0;

function checkDirection() {
  if (images.length == 1) return;
  if (touchendX - touchstartX < -50) showNextImage();
  if (touchendX - touchstartX > 50) showPreviousImage();
}

var imagesDiv = document.querySelector(".images-div");
imagesDiv.addEventListener("touchstart", (e) => {
  touchstartX = e.changedTouches[0].screenX;
});

imagesDiv.addEventListener("touchend", (e) => {
  touchendX = e.changedTouches[0].screenX;
  checkDirection();
});
