async function showCities(input) {
  const dropdown = document.querySelector(".cities-dropdown");

  if (input.value.length === 0) {
    dropdown.style.display = "none";
    input.style.borderRadius = "40px";
    return;
  }

  while (!document.querySelector(".cities-div > *")) {
    await delay(50);
  }

  dropdown.style.display = "block";
  input.style.borderRadius = "25px 25px 0 0";
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
