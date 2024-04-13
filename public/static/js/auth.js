function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function showCities() {
  while (true) {
    await delay(50);

    if (
      document.getElementsByClassName("cities-div")[0].childNodes.length > 0
    ) {
      break;
    }
  }

  const input = document.getElementsByName("city_dummy")[0];
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
      const cityInput = document.getElementsByName("city_dummy")[0];

      cityInput.value = cityName;
      document.getElementsByName("city")[0].value = cityValue;

      cityInput.style.borderRadius = "10px";
      document.getElementsByClassName("cities-dropdown")[0].style.display =
        "none";
    }
  },
);

function validateForm() {
  const form = document.forms["registerForm"].getElementsByTagName("input");
  console.log(form);
}

const targetDiv = document.getElementsByClassName("msg")[0];
const observer = new MutationObserver(async function (mutationsList, observer) {
  for (const mutation of mutationsList) {
    if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
      if (targetDiv.childNodes[0].classList[0] == "success") {
        document.getElementsByClassName("spinner")[0].style.display = "block";
        await delay(1000);
        window.location.href = "/login";
      }
    }
  }
});

const config = { childList: true, subtree: true };
observer.observe(targetDiv, config);
