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
var icon = document.querySelector("#hidden");

function toggleHamburger() {
  menu.classList.toggle("hide-menu");
  menu.classList.toggle("show-menu");

  const isHidden = icon.id === "hidden";
  icon.src = isHidden
    ? "/assets/icons/close.svg"
    : "/assets/icons/hamburger.svg";
  icon.id = isHidden ? "open" : "hidden";
}

function toggleMenu() {
  menu.classList.toggle("hide-menu");
  menu.classList.toggle("show-menu");

  const isHidden = icon.id === "hidden";

  icon.style.transform = isHidden ? "rotate(180deg)" : "rotate(0deg)";
  icon.id = isHidden ? "open" : "hidden";
}

document.addEventListener("click", (event) => {
  const target = event.target;
  const menuBtn = document.querySelector(".dropdownBtn");
  const isClickInside = menu.contains(target) || menuBtn.contains(target);

  if (!isClickInside && !menu.classList.contains("hide-menu")) {
    if (menuBtn.classList.contains("user-div")) {
      toggleMenu();
    } else {
      toggleHamburger();
    }
  }
});

function logout() {
  fetch("/logout", {
    method: "POST",
  });
}

var delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
