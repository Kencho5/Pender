function validateForm() {
  const inputs = document.querySelectorAll(".form-input");
  let isValid = true;

  inputs.forEach((input) => {
    if (!input.value || input.tagName == "DIV" && !input.textContent) {
      input.classList.add("invalid");
      isValid = false;
    }
  });

  const msg = document.querySelector(".msg");
  if (!isValid) {
    msg.innerHTML =
      '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>Fill in the form</p>';
  }
}

const inputs = document.querySelectorAll(".form-input");
inputs.forEach((input) => {
  input.addEventListener("input", function () {
    this.classList.remove("invalid");
  });

  if (input.tagName == "DIV") {
    input.addEventListener("click", function () {
      this.classList.remove("invalid");
    });
  }
});
