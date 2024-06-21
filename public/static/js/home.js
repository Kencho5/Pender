function search(form) {
  let filters = JSON.parse(localStorage.getItem("filters"));

  if (filters) {
    form.setAttribute("hx-vals", JSON.stringify(cleanFilters(filters)));
  }

  return true;
}

function scrollCategories(dir) {
  const images = htmx.find(".images");
  let scrollValue = (dir === "left")
    ? -(images.scrollWidth - images.clientWidth)
    : images.scrollWidth - images.clientWidth;

  images.scrollTo({
    left: scrollValue,
    behavior: "smooth",
  });
}
