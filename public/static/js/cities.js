async function showCities() {
  while (!document.querySelector(".cities-div > *")) {
    await delay(50);
  }

  const input = document.getElementsByName("city_dummy")[0];
  document.getElementsByClassName("cities-dropdown")[0].style.display = "block";

  input.style.borderRadius = "25px 25px 0px 0px";
  input.style.outline = "none";
}

var citiesDiv = document.querySelector(".cities-div");
if (citiesDiv) {
  citiesDiv.addEventListener("click", function (event) {
    if (event.target.tagName === "P") {
      var cityName = event.target.textContent;
      var cityValue = event.target.classList[0];
      const cityInput = document.getElementsByName("city_dummy")[0];

      cityInput.value = cityName;
      document.getElementsByName("city")[0].value = cityValue;

      cityInput.style.borderRadius = "40px";
      document.getElementsByClassName("cities-dropdown")[0].style.display =
        "none";
    }
  });
}
