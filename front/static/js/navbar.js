// INIT FUNCTION
window.onload = function () {
  activeRoute();
};

function activeRoute() {
  const route =
    window.location.pathname == "/"
      ? "home"
      : window.location.pathname.slice(0);

  const links = document.getElementsByClassName("navbar-links")[0].childNodes;

  links.forEach((link) => {
    if (link.textContent.toLowerCase() == route) {
      link.classList.add("active");
    }
  });
}
function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

document.getElementById("hamburgerMenu").addEventListener("click", async () => {
  const contextMenu = document.getElementById("contextMenu");

  if (!contextMenu.classList.contains("show-context-menu")) {
    contextMenu.classList.remove("hide-context-menu");
    contextMenu.classList.add("show-context-menu");
  } else {
    contextMenu.classList.add("hide-context-menu");
    await delay(100);
    contextMenu.classList.remove("show-context-menu");
  }
});
