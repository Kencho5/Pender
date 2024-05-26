localStorage.removeItem("filters");

function toggleFilters() {
  document.querySelector(".modal").classList.toggle("show-modal");
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

  if (!chip.classList.contains("input-div")) {
    for (const chips of chip.parentNode.children) {
      chips.classList.remove("active-chip");
    }

    chip.classList.toggle("active-chip");
  }
}

function toggleAgeDropdown() {
  document.querySelector(".age-dropdown").classList.toggle("show-age");
}

function selectAgeType(event) {
  const age = event.target;

  if (age.classList.length == 0) {
    document.querySelector(".age-selector").childNodes[0].textContent =
      ` ${age.textContent} `;
    document.querySelector(".age-dropdown").classList.remove("show-age");
  }
}
