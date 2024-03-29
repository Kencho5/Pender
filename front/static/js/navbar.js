// INIT FUNCTION
window.onload = function () {
  activeRoute();
};

function activeRoute() {
  const route = window.location.pathname == "/"
    ? "home"
    : window.location.pathname.slice(0).replace("/", "");

  const links = document.getElementsByClassName("navbar-links")[0].childNodes;

  links.forEach((link) => {
    if (link.id == route) {
      link.classList.add("active");
    }
  });
}
function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

document.getElementById("hamburgerMenu").addEventListener("click", async () => {
  const contextMenu = document.getElementById("contextMenu");
  const menuIcon = document.getElementById("menu-icon");

  if (!contextMenu.classList.contains("show-context-menu")) {
    contextMenu.classList.remove("hide-context-menu");
    contextMenu.classList.add("show-context-menu");

    menuIcon.classList = "fa-solid fa-xmark";
  } else {
    contextMenu.classList.add("hide-context-menu");
    menuIcon.classList = "fa-solid fa-bars";

    await delay(100);
    contextMenu.classList.remove("show-context-menu");
  }
});

async function reloadPage(ms) {
  await delay(ms);
  location.reload();
}
