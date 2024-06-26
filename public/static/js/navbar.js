document.body.addEventListener("htmx:afterSwap", function (evt) {
  if (evt.target.tagName != "BODY") {
    return;
  }
  window.scrollTo(0, -100);
});

async function changeLang(lang) {
  lang = (lang === "GEO") ? "ENG" : "GEO";

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
