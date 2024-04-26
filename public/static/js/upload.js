let step = 1;

function validateForm() {
  const inputs = document.querySelectorAll(".form-input");
  let isValid = true;

  inputs.forEach((input) => {
    if (input.parentNode.parentNode.id !== `step${step}`) return;

    const isEmptyInput = (input.tagName === "DIV" && !input.innerHTML) ||
      (input.tagName === "INPUT" && !input.value && input.type !== "hidden");

    if (isEmptyInput) {
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

function handleInputChange() {
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
}

function handleDropdownSelection() {
  const dropdownContent = document.querySelectorAll(".dropdown-content");
  dropdownContent.forEach(function (content) {
    content.addEventListener("click", function (event) {
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
    });
  });
}

function toggleDropdown() {
  const dropdownContent = document.querySelectorAll(".dropdown-content");
  dropdownContent.forEach(function (content) {
    content.classList.toggle("active-dropdown");
  });
}

function changeStep() {
  if (step >= 6) return;
  const prevStep = document.querySelector(`#step${step - 1}`);
  const activeStep = document.querySelector(`#step${step}`);

  prevStep.style.display = "none";
  activeStep.style.display = "block";
  activeStep.offsetHeight; // Force reflow to apply the opacity transition
  activeStep.style.opacity = "1";
}

function handleFileUpload() {
  const fileInput = document.getElementById("fileInput");
  const msg = document.querySelector(".msg");
  const photosDiv = document.getElementById("photosDiv");
  const imageContainer = document.getElementById("imageContainer");

  photosDiv.addEventListener("click", function () {
    fileInput.click();
  });

  fileInput.addEventListener("change", function () {
    const files = this.files;
    if (files.length !== 3) {
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
        const img = document.createElement("img");
        img.src = URL.createObjectURL(file);
        img.width = 200;
        img.height = 150;
        imageContainer.appendChild(img);
      }
    }
  });
}

function toBase64(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(reader.result);
    reader.onerror = reject;
  });
}

async function compressImage(file, { quality = 1, type = file.type }) {
  const imageBitmap = await createImageBitmap(file);

  const canvas = document.createElement("canvas");
  canvas.width = imageBitmap.width;
  canvas.height = imageBitmap.height;
  const ctx = canvas.getContext("2d");
  ctx.drawImage(imageBitmap, 0, 0);

  const base64 = canvas.toDataURL(type, quality);

  return base64;
}

function handleFormSubmit() {
  document.querySelector(".auth-form").addEventListener(
    "submit",
    async function (event) {
      event.preventDefault();
      if (!validateForm(step)) return;

      const formData = new FormData(this);
      const body = {};
      const promises = [];

      formData.forEach(function (value, key) {
        if (key === "photos") {
          const compressedFile = compressImage(value, {
            quality: 0.4,
            type: "image/jpeg",
          });
          promises.push(
            compressedFile.then(function (result) {
              if (!body["photos"]) body["photos"] = [result];
              else body["photos"].push(result);
            }),
          );
        } else {
          body[key] = value;
        }
      });

      await Promise.all(promises);
      uploadPost(body);
    },
  );
}

function uploadPost(body) {
  const xhr = new XMLHttpRequest();
  xhr.open("POST", "/upload", true);

  xhr.upload.addEventListener("progress", (event) => {
    if (event.lengthComputable) {
      const percent = (event.loaded / event.total) * 100;
      const progressDiv = document.querySelector(".progress-div");
      const progressBar = document.querySelector(".progress");
      const progressInfo = document.querySelector(".progress-percentage");

      progressDiv.style.display = "block";
      progressBar.style.width = percent;
      progressInfo.textContent = `${percent}%`;
    }
  });

  xhr.onload = async function () {
    const progressStatus = document.querySelector(".progress-status");
    progressStatus.textContent = "Done!";

    const data = JSON.parse(xhr.responseText);
    const { error, post_id } = data;

    const msg = document.querySelector(".msg");
    if (error) {
      msg.innerHTML =
        '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>Error uploading post</p>';
    } else {
      msg.innerHTML =
        `<p class='success'><i class="fa-solid fa-circle-check"></i>Upload complete!</p>`;
      await delay(700);
      // window.location.href = `/post/${post_id}`;
    }
  };

  xhr.send(JSON.stringify(body));
}

function init() {
  handleInputChange();
  handleDropdownSelection();
  handleFileUpload();
  handleFormSubmit();
}

document.addEventListener("DOMContentLoaded", init);
