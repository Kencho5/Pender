document.body.addEventListener("htmx:beforeSwap", function (event) {
  if (event.target.tagName != "BODY") {
    return;
  }

  const mainContent = document.querySelector(".main-content");
  animateOut(mainContent, 0.3);
});

document.body.addEventListener("htmx:afterSwap", function (event) {
  if (event.target.tagName != "BODY") {
    return;
  }
  window.scrollTo(0, -100);

  const mainContent = document.querySelector(".main-content");
  mainContent.style.opacity = 0; // Set initial opacity
  animateIn(mainContent, 0.3);
});

async function changeLang(lang) {
  lang = lang === "GEO" ? "ENG" : "GEO";

  await fetch(`/api/set_language/${lang}`, {
    method: "POST",
  });

  htmx.ajax("GET", window.location.href, {
    target: "body",
    swap: "outerHTML",
  });
}

function logout() {
  fetch("/logout", {
    method: "POST",
  });
}

var delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
