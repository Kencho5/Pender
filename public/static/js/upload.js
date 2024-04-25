let step = 1;

function validateForm() {
  const inputs = document.querySelectorAll(".form-input");
  let isValid = true;

  inputs.forEach((input) => {
    if (
      input.parentNode.parentNode.id != `step${step}`
    ) return;

    if (
      (input.tagName == "DIV" && !input.innerHTML) ||
      (input.tagName == "INPUT" && !input.value) &&
        input.type != "hidden"
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
    step++;
    changeStep();
  }

  return isValid;
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

const dropdownContent = document.querySelectorAll(".dropdown-content");
dropdownContent.forEach(function (content) {
  content.addEventListener("click", function (event) {
    if (
      event.target.getAttribute("data-target") == "post_type" &&
      event.target.id == "selling"
    ) {
      const priceInputs = Array.from(
        document.getElementsByClassName("price-hidden"),
      );
      document.getElementsByName("price")[0].type = "text";

      priceInputs.forEach((element) => {
        element.style.display = "block";
      });
    }
    if (event.target.tagName === "P") {
      const selectedText = event.target.textContent;
      const selectedId = event.target.id;
      const selectedData = event.target.getAttribute("data-target");

      document.querySelector(`div[target="${selectedData}"]`).textContent =
        selectedText;
      document.getElementsByName(selectedData)[0].value = selectedId;
      toggleDropdown();
    }
  });
});

function toggleDropdown() {
  dropdownContent.forEach(function (content) {
    content.classList.toggle(
      "active-dropdown",
    );
  });
}

function changeStep() {
  if (step >= 6) return;
  const prevStep = document.querySelector(`#step${step - 1}`);
  const activeStep = document.querySelector(`#step${step}`);

  prevStep.style.display = "none";

  activeStep.style.display = "block";
  activeStep.offsetHeight;
  activeStep.style.opacity = "1";
}

document.addEventListener("DOMContentLoaded", function () {
  const fileInput = document.getElementById("fileInput");
  const msg = document.querySelector(".msg");
  const photosDiv = document.getElementById("photosDiv");
  const imageContainer = document.getElementById("imageContainer");

  photosDiv.addEventListener("click", function () {
    fileInput.click();
  });
  fileInput.addEventListener("change", function () {
    const files = this.files;
    if (files.length != 3) {
      this.value = "";
      document.querySelector('[name="photos"]').value = "";

      msg.innerHTML =
        '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>3 Photos Min/Max!</p>';
      return;
    }

    msg.innerHTML = "";
    imageContainer.innerHTML = "";
    photosDiv.innerHTML = "3 Photos Selected";

    for (let i = 0; i < this.files.length; i++) {
      const file = this.files[i];

      if (file.type.startsWith("image/")) {
        // CREATE IMAGE
        const img = document.createElement("img");
        img.src = URL.createObjectURL(file);
        img.width = 200;
        img.height = 150;

        imageContainer.appendChild(img);
      }
    }
  });
});

const toBase64 = (file) =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(reader.result);
    reader.onerror = reject;
  });

document.querySelector(".auth-form").addEventListener(
  "submit",
  async function (event) {
    event.preventDefault();
    if (!validateForm()) return;

    var formData = new FormData(this);

    var body = {};
    var promises = [];

    formData.forEach(function (value, key) {
      if (key == "photos") {
        const bytes = toBase64(value);
        promises.push(bytes.then(function (result) {
          if (!body["photos"]) body["photos"] = [result];
          else body["photos"].push(result);
        }));
      } else {
        body[key] = value;
      }
    });

    await Promise.all(promises);

    fetch("/upload", {
      method: "POST",
      body: JSON.stringify(body),
    }).then(function (response) {
      return response.json();
    }).then(function (data) {
      const { error, post_id } = data;

      if (error) {
        const msg = document.querySelector(".msg");
        msg.innerHTML = error;
      } else {
        window.location.href = `/post/${post_id}`;
      }
    });
  },
);
