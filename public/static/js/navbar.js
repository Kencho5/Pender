document.body.addEventListener("htmx:afterSwap", function (evt) {
  if (evt.target.tagName != "BODY") {
    return;
  }
  window.scrollTo(0, 0);

  const mainContent = document.querySelector(".main-content");
  mainContent.style.opacity = 0;

  mainContent.style.transition = "none";

  mainContent.offsetHeight;

  mainContent.style.transition = "opacity 0.2s ease-in-out";
  mainContent.style.opacity = 1;
});

function changeLang(lang) {
  lang = (lang === "GEO") ? "ENG" : "GEO";

  fetch(`/api/set_language/${lang}`, {
    method: "POST",
  });
}

var delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
