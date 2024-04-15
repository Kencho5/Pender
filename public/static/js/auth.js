async function showCities() {
  while (!document.querySelector(".cities-div > *")) {
    await delay(50);
  }

  const input = document.getElementsByName("city_dummy")[0];
  document.getElementsByClassName("cities-dropdown")[0].style.display = "block";

  input.style.borderRadius = "10px 10px 0px 0px";
  input.style.outline = "none";
}

const citiesDiv = document.querySelector(".cities-div");
if (citiesDiv) {
  citiesDiv.addEventListener("click", function (event) {
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
  });
}

function validateForm(formName) {
  const formInputs = Array.from(
    document.forms[formName].querySelectorAll("input"),
  );
  let isValid = true;

  formInputs.forEach((input) => {
    if (!input.value) {
      input.classList.add("invalid");
      isValid = false;
    } else {
      input.classList.remove("invalid");
    }
  });

  if (!isValid) {
    document.querySelector(".msg").innerHTML =
      '<p class="error">Fill in the form</p>';
  }

  return isValid;
}

const targetDiv = document.querySelector(".msg");
const observer = new MutationObserver(async function (mutationsList) {
  for (const mutation of mutationsList) {
    if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
      if (targetDiv.childNodes[0].classList.contains("success")) {
        document.querySelector(".spinner").style.display = "block";
        await delay(1000);
        window.location.href = "/login";
      }
    }
  }
});

const config = { childList: true, subtree: true };
observer.observe(targetDiv, config);

const inputs = document.querySelectorAll("input");
inputs.forEach((input) => {
  input.addEventListener("input", function () {
    this.classList.remove("invalid");
  });
});
