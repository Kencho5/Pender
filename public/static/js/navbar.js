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

async function changeLang(lang) {
  lang = (lang === "GEO") ? "ENG" : "GEO";

  await fetch(`/api/set_language/${lang}`, {
    method: "POST",
  });

  htmx.ajax("GET", window.location.pathname, {
    target: "body",
    swap: "outerHTML",
  });
}

var menu = document.querySelector(".nav-dropdown");
var icon = document.querySelector("#hidden");

function toggleMenu() {
  menu.classList.toggle("hide-menu");
  menu.classList.toggle("show-menu");

  toggleIcon();
}

function toggleIcon() {
  const isHidden = icon.id === "hidden";
  if (icon.classList.length == 0) {
    icon.style.transform = isHidden ? "rotate(180deg)" : "rotate(0deg)";
  } else {
    icon.src = isHidden
      ? "/assets/icons/close.svg"
      : "/assets/icons/hamburger.svg";
  }
  icon.id = isHidden ? "open" : "hidden";
}

document.addEventListener("click", (event) => {
  const target = event.target;
  const menuBtn = document.querySelector(".dropdownBtn");
  const isClickInside = menu.contains(target) || menuBtn.contains(target);

  if (!isClickInside && !menu.classList.contains("hide-menu")) {
    toggleMenu();
  }
});

function logout() {
  fetch("/logout", {
    method: "POST",
  });
}

var delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
