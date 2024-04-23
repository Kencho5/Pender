function validateForm() {
  const inputs = document.querySelectorAll(".form-input");
  let isValid = true;

  inputs.forEach((input) => {
    if (
      (input.tagName == "DIV" && !input.innerHTML) ||
      (input.tagName == "INPUT" && !input.value)
    ) {
      input.classList.add("invalid");
      isValid = false;
    }
  });

  const msg = document.querySelector(".msg");
  if (!isValid) {
    msg.innerHTML =
      '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>Fill in the form</p>';
  } else {
    msg.innerHTML = "";
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

const dropdownContent = document.querySelector(".dropdown-content");
dropdownContent.addEventListener("click", function (event) {
  if (event.target.tagName === "P") {
    var selectedText = event.target.textContent;
    var selectedId = event.target.id;

    document.querySelector(".dropdown-div").textContent = selectedText;
    document.querySelector(".dropdown-input").value = selectedId;
    toggleDropdown();
  }
});

function toggleDropdown() {
  dropdownContent.classList.toggle(
    "active-dropdown",
  );
}
