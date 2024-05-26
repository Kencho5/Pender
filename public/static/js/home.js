localStorage.removeItem("filters");

function toggleFilters() {
  document.querySelector(".modal").classList.toggle("show-modal");
}

function saveFilters(form, ev) {
  ev.preventDefault();

  const formData = new FormData(form);

  const filters = {};
  for (const [key, value] of formData.entries()) {
    if (!value) continue;
    filters[key] = value;
  }

  localStorage.setItem("filters", JSON.stringify(filters));
}

function search(form) {
  const filters = JSON.parse(localStorage.getItem("filters"));

  if (filters) {
    form.setAttribute("hx-vals", JSON.stringify(filters));
  }

  return true;
}

function selectChip(event) {
  const chip = event.target;
  const filters = JSON.parse(localStorage.getItem("filters")) || {};

  if (!chip.classList.contains("input-div")) {
    for (const chips of chip.parentNode.children) {
      chips.classList.remove("active-chip");
    }

    filters[chip.parentNode.getAttribute("input-name")] = chip.id;
    chip.classList.toggle("active-chip");
  }

  localStorage.setItem("filters", JSON.stringify(filters));
}

function toggleAgeDropdown() {
  document.querySelector(".age-dropdown").classList.toggle("show-age");
}

function selectAgeType(event) {
  const age = event.target;
  const filters = JSON.parse(localStorage.getItem("filters")) || {};

  if (age.classList.length == 0) {
    filters[age.parentNode.getAttribute("input-name")] = age.id;
    document.querySelector(".age-selector").childNodes[0].textContent =
      ` ${age.textContent} `;
  }

  localStorage.setItem("filters", JSON.stringify(filters));
  document.querySelector(".age-dropdown").classList.remove("show-age");
}

function selectCity(event) {
  const city = event.target;
  const filters = JSON.parse(localStorage.getItem("filters")) || {};

  if (!city.classList.contains("cities-div")) {
    filters[city.parentNode.getAttribute("input-name")] = city.classList[0];
  }
  localStorage.setItem("filters", JSON.stringify(filters));
}
