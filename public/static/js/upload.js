// form-validation.js
function validateForm(step) {
  const inputs = document.querySelectorAll(".form-input");
  let isValid = true;

  inputs.forEach((input) => {
    if (input.parentNode.parentNode.id !== `step${step}`) return;

    if (
      (input.tagName === "DIV" && !input.innerHTML) ||
      (input.tagName === "INPUT" && !input.value && input.type !== "hidden")
    ) {
      input.classList.add("invalid");
      isValid = false;
    }
  });

  return isValid;
}

// ui-helpers.js
function showFormMessage(message, type) {
  const msg = document.querySelector(".msg");
  msg.innerHTML = `<p class="${type}">${message}</p>`;
}

function toggleDropdown() {
  dropdownContent.forEach((content) => {
    content.classList.toggle("active-dropdown");
  });
}

function changeStep(step) {
  if (step >= 6) return;
  const prevStep = document.querySelector(`#step${step - 1}`);
  const activeStep = document.querySelector(`#step${step}`);

  prevStep.style.display = "none";

  activeStep.style.display = "block";
  activeStep.offsetHeight;
  activeStep.style.opacity = "1";
}

// event-handlers.js
const inputs = document.querySelectorAll(".form-input");
inputs.forEach((input) => {
  input.addEventListener("input", function () {
    this.classList.remove("invalid");
  });

  if (input.tagName === "DIV") {
    input.addEventListener("click", function () {
      this.classList.remove("invalid");
    });
  }
});

const dropdownContent = document.querySelectorAll(".dropdown-content");
dropdownContent.forEach(function (content) {
  content.addEventListener("click", function (event) {
    handleDropdownClick(event);
  });
});

function handleDropdownClick(event) {
  if (
    event.target.getAttribute("data-target") === "post_type" &&
    event.target.id === "selling"
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
}

// image-upload.js
document.addEventListener("DOMContentLoaded", function () {
  const fileInput = document.getElementById("fileInput");
  const photosDiv = document.getElementById("photosDiv");
  const imageContainer = document.getElementById("imageContainer");

  photosDiv.addEventListener("click", function () {
    fileInput.click();
  });
  fileInput.addEventListener("change", handleFileUpload);
});

function handleFileUpload() {
  const files = this.files;
  if (files.length !== 3) {
    this.value = "";
    document.querySelector('[name="photos"]').value = "";

    showFormMessage(
      '<i class="fa-solid fa-circle-exclamation"></i>3 Photos Min/Max!',
      "error",
    );
    return;
  }

  showFormMessage("", "");
  imageContainer.innerHTML = "";
  photosDiv.innerHTML = "3 Photos Selected";

  for (let i = 0; i < this.files.length; i++) {
    const file = this.files[i];

    if (file.type.startsWith("image/")) {
      const img = document.createElement("img");
      img.src = URL.createObjectURL(file);
      img.width = 200;
      img.height = 150;

      imageContainer.appendChild(img);
    }
  }
}

// image-compression.js
const compressImage = async (file, { quality = 1, type = file.type }) => {
  const imageBitmap = await createImageBitmap(file);

  const canvas = document.createElement("canvas");
  canvas.width = imageBitmap.width;
  canvas.height = imageBitmap.height;
  const ctx = canvas.getContext("2d");
  ctx.drawImage(imageBitmap, 0, 0);

  const base64 = canvas.toDataURL(type, quality);

  return base64;
};

// form-submission.js
document.querySelector(".auth-form").addEventListener(
  "submit",
  async function (event) {
    event.preventDefault();
    const step = 1;
    if (!validateForm(step)) return;

    var formData = new FormData(this);

    var body = {};
    var promises = [];

    formData.forEach(function (value, key) {
      if (key === "photos") {
        const compressedFile = compressImage(value, {
          quality: 1,
          type: "image/jpeg",
        });
        promises.push(compressedFile.then(function (result) {
          if (!body["photos"]) body["photos"] = [result];
          else body["photos"].push(result);
        }));
      } else {
        body[key] = value;
      }
    });

    await Promise.all(promises);
    uploadPost(body);
  },
);

async function uploadPost(body) {
  const response = await fetch("/upload", {
    method: "POST",
    body: JSON.stringify(body),
  });
  let total = 0;

  for await (const chunk of response.body) {
    total += chunk.length;
    console.log(total);
  }
  // .then(function (response) {
  //   return response.json();
  // })
  // .then(function (data) {
  //   const { error, post_id } = data;
  //
  //   if (error) {
  //     showFormMessage(error, "error");
  //   } else {
  //     // window.location.href = `/post/${post_id}`;
  //   }
  // });
}
