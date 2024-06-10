var searchParams = new URLSearchParams(window.location.search);
document.getElementsByName("breed")[0].value = searchParams.get("breed");
