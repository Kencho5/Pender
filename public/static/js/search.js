var searchParams = new URLSearchParams(window.location.search);
document.getElementsByName("breed")[0].value = searchParams.get("breed");

function updateFiltersNum() {
  const filters = JSON.parse(localStorage.getItem("filters")) || {};
  let filtersLength = Object.keys(filters).filter((key) => {
    let value = filters[key];
    if (Array.isArray(value) && value != "all") {
      return value.length > 0;
    } else {
      return value !== "";
    }
  }).length;

  if (filtersLength <= 1) return;

  const filtersNum = htmx.find(".filters-num");

  filtersNum.style.display = "flex";
  filtersNum.textContent = filtersLength - 1;
  htmx.addClass(htmx.find(".filters-btn"), "active-num");
}

updateFiltersNum();
