// localStorage.removeItem("filters");

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
