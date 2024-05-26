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
    // for (const chips of chip.parentNode.children) {
    //   chips.classList.remove("active-chip");
    // }

    chip.classList.toggle("active-chip");
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

function saveFilters(form, event) {
  event.preventDefault();

  const formData = new FormData(form);
  const filters = {};

  // Get the values of input fields
  for (const [name, value] of formData.entries()) {
    filters[name] = value;
  }

  // Get the values of selected chips
  const chipInputs = form.querySelectorAll(".input-div[input-name]");
  chipInputs.forEach((chipInput) => {
    const inputName = chipInput.getAttribute("input-name");
    const selectedChips = Array.from(
      chipInput.querySelectorAll(".chip.active-chip"),
    ).map((chip) => chip.id);
    if (selectedChips.length > 0) {
      filters[inputName] = selectedChips;
    }
  });

  // Get age type
  filters["ageType"] = document.querySelector(".age-selector").id;

  console.log(filters);

  toggleFilters();
}
