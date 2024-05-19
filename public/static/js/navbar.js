activeRoute();

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

function activeRoute() {
  let route = window.location.pathname == "/"
    ? "home"
    : window.location.pathname.slice(0).replace("/", "");
  if (route == "register") route = "login";
  const links = document.getElementsByClassName("navbar-links")[0].childNodes;

  links.forEach((link) => {
    if (link.id == route) link.classList.add("active-navbar-link");
  });

  const mobileLinks = Array.from(document.getElementsByClassName("menu-link"));
  mobileLinks.forEach((link) => {
    if (link.id == route) link.classList.add("active-menu-link");
  });
}

document.getElementById("hamburgerMenu").addEventListener("click", async () => {
  const mobileMenu = document.getElementById("mobileMenu");
  const menuIcon = document.getElementById("menu-icon");

  if (!mobileMenu.classList.contains("show-mobile-menu")) {
    mobileMenu.classList.remove("hide-mobile-menu");
    mobileMenu.classList.add("show-mobile-menu");

    menuIcon.classList = "fa-solid fa-xmark";
  } else {
    mobileMenu.classList.add("fade-out");
    menuIcon.classList = "fa-solid fa-bars";

    await delay(300);

    mobileMenu.classList.add("hide-mobile-menu");
    mobileMenu.classList.remove("fade-out");
    mobileMenu.classList.remove("show-mobile-menu");
  }
});

function showDropdown() {
  const dropdownContent = document.querySelectorAll(".animal-dropdown-content");
  dropdownContent.forEach(function (content) {
    content.classList.toggle("active-animal-dropdown");
  });
}

function selectAnimal(event) {
  if (event.target.tagName === "P") {
    const selectedClass = event.target.classList[0];
    const selectedId = event.target.id;

    document.querySelector(".animal-input").value = selectedId;
    document.querySelector(".animal-dropdown > i").className =
      `fa fa-${selectedClass}`;

    showDropdown();
  }
}
document.querySelector(".animal-dropdown-content").addEventListener(
  "click",
  selectAnimal,
);

async function reloadPage(ms) {
  await delay(ms);
  location.reload();
}

var delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
