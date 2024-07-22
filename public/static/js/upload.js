var form = {};
var step = 1;
localStorage.setItem("step", step);

function setData(key, value) {
  form[key] = value;
}

function showData() {
  if (Object.keys(form).length == 0) return;

  for (const [key, value] of Object.entries(form)) {
    const element = htmx.find(`#${value}`);

    if (!element) continue;
    element.classList.add("active-chip");
  }
}

document.body.addEventListener("htmx:afterSettle", function (event) {
  if (!event.target.className.includes("info")) return;
  showData();
});

function changeStep(dir) {
  if (dir == "next") step++;
  else step--;
  step = Math.max(1, Math.min(step, 4));

  if (localStorage.getItem("step") != step - 1) return;

  const icon = htmx.find(`#step${step - 1}`);
  icon.children[0].style.backgroundColor = "#6172F3";
  icon.children[1].classList.add("hidden");
  icon.children[2].classList.remove("hidden");
  icon.parentNode.classList.add("steps__active");

  localStorage.setItem("step", step);
}
