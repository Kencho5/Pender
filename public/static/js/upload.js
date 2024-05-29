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

function selectChip(event) {
  const chip = event.target;
  const priceRow = document.querySelector(".price-row");

  if (!chip.classList.contains("input-div")) {
    if (chip.parentNode.getAttribute("input-name") == "post_type") {
      priceRow.style.display = "none";
      priceRow.querySelector("input").classList.remove("upload-input");
    }

    Array.from(chip.parentNode.children).forEach((element) => {
      element.classList.remove("active-upload-chip");
    });
    chip.classList.toggle("active-upload-chip");
  }

  if (chip.id == "selling") {
    priceRow.style.display = "block";
    priceRow.querySelector("input").classList = "upload-input";
  }
}

function toggleAgeDropdown() {
  document.querySelector(".age-dropdown").classList.toggle("show-age");
}

function selectAgeType(event) {
  const age = event.target;

  if (age.classList.length == 0) {
    const ageSelector = document.querySelector(".age-selector").childNodes[0];
    const ageDropdown = document.querySelector(".age-dropdown");

    ageSelector.textContent = ` ${age.textContent} `;
    ageSelector.parentNode.id = age.id;
    ageDropdown.classList.remove("show-age");
  }
}

function validateForm() {
  let isValid = true;
  const msg = document.querySelector(".msg");
  const chips = document.querySelectorAll(".active-upload-chip");
  const inputs = document.querySelectorAll(".upload-input");

  inputs.forEach((input) => {
    console.log(input);
    if (!input.value) isValid = false;
  });

  if (chips.length < 3) isValid = false;

  if (!isValid) {
    msg.style.display = "block";
  }
}

function upload() {
  if (!validateForm()) return;
}
