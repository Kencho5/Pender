var detailsDiv = document.querySelector(".details-div");

function triggerGlow() {
  detailsDiv.classList.add("saved");
}

detailsDiv.addEventListener("animationend", () => {
  detailsDiv.classList.remove("saved");
});
