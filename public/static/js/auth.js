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
      '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>Fill in the form</p>';
  }

  return isValid;
}

const spinner = document.querySelector(".spinner");
if (spinner) {
  const targetDiv = document.querySelector(".msg");
  const observer = new MutationObserver(async function (mutationsList) {
    for (const mutation of mutationsList) {
      if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
        if (targetDiv.childNodes[0].classList.contains("success")) {
          spinner.style.display = "block";
          await delay(1000);
          window.location.href = "/login";
        }
      }
    }
  });

  const config = { childList: true, subtree: true };
  observer.observe(targetDiv, config);
}

const inputs = document.querySelectorAll("input");
inputs.forEach((input) => {
  input.addEventListener("input", function () {
    this.classList.remove("invalid");
  });
});
