// INIT FUNCTION
window.onload = function () {
  activeRoute();
  languageSelectorListener("languageSelector");
  languageSelectorListener("languageSelectorMobile");
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

function languageSelectorListener(id) {
  document.getElementById(id).addEventListener("click", async () => {
    const contextMenu = document.getElementById("languageMenu");

    if (!contextMenu.classList.contains("show-language-menu")) {
      contextMenu.classList.remove("hide-language-menu");
      contextMenu.classList.add("show-language-menu");
    } else {
      contextMenu.classList.add("hide-language-menu");
      await delay(100);
      contextMenu.classList.remove("show-language-menu");
    }
  });
}
