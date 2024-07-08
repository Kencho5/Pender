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

  return isValid;
}

var inputs = document.querySelectorAll("input");
inputs.forEach((input) => {
  input.addEventListener("input", function () {
    this.classList.remove("invalid");
  });
});

async function loginStatus() {
  await delay(50);
  const state = document.querySelector(".msg").childNodes[0].classList[0];
  if (state != "success") return;

  htmx.ajax("GET", "/profile", {
    target: document.body,
    swap: "outerHTML",
  }).then(() => {
    history.pushState(null, "", "/profile");
  });
}

async function registerStatus() {
  await delay(50);
  const form = document.querySelector(".auth-form");
  const successDiv = document.querySelector(".success-div");
  const state = document.querySelector(".msg").firstChild;
  if (!state || state.classList[0] != "success") return;

  form.style.display = "none";
  successDiv.style.display = "block";
  window.scrollTo({ top: 0, behavior: "smooth" });
}
