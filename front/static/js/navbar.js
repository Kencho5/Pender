function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function animateContextMenuLinks() {
  const links = document.querySelectorAll(".context-link");

  for (let index = 0; index < links.length; index++) {
    await delay(30);
    links[index].classList.add("show-link");
  }
}

async function animateHideContextMenuLinks() {
  const links = document.querySelectorAll(".context-link");
  const reversedLinks = Array.from(links).reverse();

  for (let index = 0; index < reversedLinks.length; index++) {
    await delay(30);
    reversedLinks[index].classList.remove("show-link");
  }

  await delay(150);
  contextMenu.classList.toggle("show-context-menu");
  contextMenu.classList.toggle("hidden");
}

document.getElementById("hamburgerMenu").addEventListener("click", () => {
  const contextMenu = document.getElementById("contextMenu");

  if (contextMenu.classList.contains("hidden")) {
    contextMenu.classList.toggle("show-context-menu");
    contextMenu.classList.toggle("hidden");

    animateContextMenuLinks();
  } else {
    animateHideContextMenuLinks();
  }
});
