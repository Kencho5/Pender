function toggleFilters() {
  document.querySelector(".modal").classList.toggle("hide-modal");
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

  if (!chip.classList.contains("input-div")) {
    for (const chips of event.target.parentNode.children) {
      chips.classList.remove("active-chip");
    }

    chip.classList.toggle("active-chip");
  }
}
