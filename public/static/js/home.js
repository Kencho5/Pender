function search(form) {
  let filters = JSON.parse(localStorage.getItem("filters"));

  if (filters) {
    form.setAttribute("hx-vals", JSON.stringify(cleanFilters(filters)));
  }

  return true;
}

function slide(dir, el) {
  const images = htmx.find(el);
  const scrollAmount = images.clientWidth / 2;
  let scrollValue;

  if (dir === "left") {
    scrollValue = images.scrollLeft - scrollAmount;
  } else {
    scrollValue = images.scrollLeft + scrollAmount;
  }

  images.scrollTo({
    left: scrollValue,
    behavior: "smooth",
  });
}
