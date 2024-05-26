localStorage.removeItem("filters");

function toggleFilters() {
  document.querySelector(".modal").classList.toggle("show-modal");
}

function search(form) {
  let filters = JSON.parse(localStorage.getItem("filters"));

  if (filters) {
    form.setAttribute("hx-vals", JSON.stringify(cleanFilters(filters)));
  }

  return true;
}

function cleanFilters(filters) {
  return Object.fromEntries(
    Object.entries(filters).filter(([_, value]) => {
      if (typeof value === "string") {
        return value.trim() !== "";
      } else if (Array.isArray(value)) {
        return value.length > 0;
      }
      return true;
    }),
  );
}

function selectChip(event) {
  const chip = event.target;

  if (!chip.classList.contains("input-div")) {
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

  localStorage.setItem("filters", JSON.stringify(filters));

  toggleFilters();
}
