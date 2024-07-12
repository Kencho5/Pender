function search(form) {
  let filters = JSON.parse(localStorage.getItem("filters"));

  if (filters) {
    form.setAttribute("hx-vals", JSON.stringify(cleanFilters(filters)));
  }

  return true;
}

function slide(dir, el) {
  const images = htmx.find(el);
  let scrollValue = (dir === "left")
    ? -(images.scrollWidth - images.clientWidth)
    : images.scrollWidth - images.clientWidth;

  images.scrollTo({
    left: scrollValue,
    behavior: "smooth",
  });
}
