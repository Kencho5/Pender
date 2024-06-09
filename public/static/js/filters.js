var filters = JSON.parse(localStorage.getItem("filters"));
if (filters) {
  for (let [key, value] of Object.entries(filters)) {
    const input = document.querySelector(`[input-name="${key}"]`);

    if (input.classList.contains("input-div")) {
      for (const chip of input.children) {
        if (value.includes(chip.id)) chip.classList.add("active-chip");
      }
    } else if (key == "city") {
      const city = document.querySelectorAll(`[input-name="${key}"]`);

      city[0].value = value.charAt(0).toUpperCase() + value.slice(1);
      city[1].value = value;
    } else if (key.includes("price")) {
      input.value = value;
    }
  }
}

function toggleFilters() {
  document.querySelector(".modal").classList.toggle("show-modal");
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

function saveFilters() {
  const form = document.querySelector("form[name='filtersForm']");

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

function clearFilters() {
  localStorage.removeItem("filters");
  htmx.ajax("GET", window.location.pathname, {
    target: "body",
    swap: "outerHTML",
  });
}

var filtersModal = document.getElementById("filtersModal");
filtersModal.addEventListener("click", function (event) {
  if (event.target == filtersModal) {
    toggleFilters();
  }
});

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
