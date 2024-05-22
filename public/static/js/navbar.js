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

var menu = document.querySelector(".nav-dropdown");

function toggleHamburger(icon) {
  menu.classList.toggle("hide-menu");

  const isHidden = icon.id === "hidden";
  icon.src = isHidden
    ? "/assets/icons/close.svg"
    : "/assets/icons/hamburger.svg";
  icon.id = isHidden ? "open" : "hidden";
}

function toggleMenu(icon) {
  menu.classList.toggle("hide-menu");

  icon = icon.childNodes[5];
  const isHidden = icon.id === "hidden";

  icon.style.transform = isHidden ? "rotate(180deg)" : "rotate(0deg)";
  icon.id = isHidden ? "open" : "hidden";
}

function logout() {
  fetch("/logout", {
    method: "POST",
  });
}

var delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
