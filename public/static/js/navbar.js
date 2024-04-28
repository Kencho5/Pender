// INIT FUNCTION
window.onload = function () {
  activeRoute();
};

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

async function reloadPage(ms) {
  await delay(ms);
  location.reload();
}

const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
