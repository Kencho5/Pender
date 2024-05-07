let step = 1;

function validateForm() {
  const inputs = Array.from(document.querySelectorAll(".form-input"));
  let isValid = true;

  const isEmptyInput = (input) => {
    if (input.parentNode.parentNode.id !== `step${step}`) return false;

    return (
      (input.tagName === "DIV" && input.childNodes.length > 1 &&
        input.childNodes[1].tagName == "P") ||
      (input.tagName === "INPUT" && !input.value &&
        !input.classList.contains("hidden"))
    );
  };

  const handleInvalidInput = (input) => {
    input.classList.add("invalid");
    isValid = false;
  };

  inputs.forEach((input) => {
    if (isEmptyInput(input)) {
      handleInvalidInput(input);
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

  const handlePostTypeSelection = (event) => {
    const priceInputs = Array.from(
      document.getElementsByClassName("price-hidden"),
    );
    if (event.target.id == "selling") {
      priceInputs.forEach((element) => {
        element.classList.remove("hidden");
      });
    } else if (event.target.id != "selling") {
      priceInputs.forEach((element) => {
        element.classList.add("hidden");
      });
    }
  };

  const handleDropdownItemClick = (event) => {
    if (event.target.tagName === "P") {
      const selectedText = event.target.textContent;
      const selectedId = event.target.id;
      const selectedData = event.target.getAttribute("data-target");
      document.querySelector(`div[target="${selectedData}"]`).textContent =
        selectedText;
      document.getElementsByName(selectedData)[0].value = selectedId;
      toggleDropdown();
    }
  };

  dropdownContent.forEach((content) => {
    content.addEventListener("click", handlePostTypeSelection);
    content.addEventListener("click", handleDropdownItemClick);
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

  window.scrollTo({ top: 0, behavior: "smooth" });

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

  const handlePhotoDivClick = () => {
    fileInput.click();
  };

  const handleFileChange = () => {
    const files = fileInput.files;

    if (files.length !== 3) {
      fileInput.value = "";
      document.querySelector('[name="photos"]').value = "";
      msg.innerHTML =
        '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>3 Photos Min/Max!</p>';
      return;
    }

    msg.innerHTML = "";
    imageContainer.innerHTML = "";
    let totalSize = 0;

    for (const file of files) {
      if (file.type.startsWith("image/")) {
        const size = (file.size / (1024 * 1024)).toFixed(2);
        totalSize += parseFloat(size);

        const img = document.createElement("img");
        img.src = URL.createObjectURL(file);
        img.width = 200;
        img.height = 150;
        imageContainer.appendChild(img);
      } else {
        fileInput.value = "";
        document.querySelector('[name="photos"]').value = "";
        msg.innerHTML =
          '<p class="error"><i class="fa-solid fa-circle-exclamation"></i>Only photos allowed</p>';
        return;
      }
    }

    photosDiv.innerHTML = `${totalSize.toFixed(2)}Mb`;
  };

  photosDiv.addEventListener("click", handlePhotoDivClick);
  fileInput.addEventListener("change", handleFileChange);
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

      document.getElementById("submit-input").disabled = true;

      const formData = new FormData(this);
      const body = {};
      const promises = [];

      formData.forEach(function (value, key) {
        if (key === "photos") {
          const compressedFile = compressImage(value, {
            quality: 0.6,
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
      let threshold = 20;
      const percent = Math.floor((event.loaded / event.total) * 100);
      const progressDiv = document.querySelector(".progress-div");
      const progressBar = document.querySelector(".progress");
      const progressInfo = document.querySelector(".progress-percentage");

      if (percent >= threshold) {
        progressDiv.style.display = "block";
        progressBar.style.width = `${percent}%`;
        progressInfo.textContent = `${percent}%`;

        threshold += 20;
      }
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
      window.location.href = `/post/${post_id}`;
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
