async function showCities() {
  while (true) {
    await delay(50);

    if (
      document.getElementsByClassName("cities-div")[0].childNodes.length > 0
    ) {
      break;
    }
  }

  const input = document.getElementsByName("city")[0];
  document.getElementsByClassName("cities-dropdown")[0].style.display = "block";

  input.style.borderRadius = "10px 10px 0px 0px";
  input.style.outline = "none";
}

document.querySelector(".cities-div").addEventListener(
  "click",
  function (event) {
    if (event.target.tagName === "P") {
      var cityName = event.target.textContent;
      var cityValue = event.target.classList[0];
      const cityInput = document.getElementsByName("city")[0];

      cityInput.value = cityName;
      document.getElementsByName("city-real")[0].value = cityValue;

      cityInput.style.borderRadius = "10px";
      document.getElementsByClassName("cities-dropdown")[0].style.display =
        "none";
    }
  },
);

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
