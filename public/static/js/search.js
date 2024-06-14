var searchParams = new URLSearchParams(window.location.search);
document.getElementsByName("breed")[0].value = searchParams.get("breed");
var pageNum = searchParams.get("page") || 1;
htmx.find(".pageNum").textContent = pageNum;

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

function changePage(dir) {
  if (dir == "next") pageNum++;
  else if (pageNum - 1 > 0) pageNum--;

  htmx.find(".pageNum").textContent = pageNum;

  const params = new URLSearchParams({
    page: pageNum,
  }).toString();

  const urlWithParams = `${window.location.pathname}?${params}`;
  htmx.ajax("GET", urlWithParams, {
    target: "body",
    swap: "outerHTML",
  }).then(() => {
    history.pushState(null, "", urlWithParams);
  });
}
