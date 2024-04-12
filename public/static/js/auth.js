async function showDropdown() {
  const dropdown = document.getElementsByClassName("dropdown-content")[0];
  dropdown.classList.toggle("show");
}

document.addEventListener("DOMContentLoaded", function () {
  document.addEventListener("click", function (event) {
    if (event.target.closest(".dropdown-content")) {
      if (event.target.tagName === "P") {
        const cityInput = document.getElementsByName("city")[0];
        const cityInputDiv = document.getElementsByClassName("city-input")[0];
        const dropdown = document.getElementsByClassName("dropdown-content")[0];

        dropdown.classList.toggle("show");
        cityInput.value = event.target.classList[0];
        cityInputDiv.textContent = event.target.textContent;
      }
    }
  });
});

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
