const spinnerDiv = document.getElementsByClassName("lds-dual-ring")[0];
const msgDiv = document.getElementsByClassName("msg")[0];
const resetForm = document.getElementsByName("resetForm")[0];
const resetCodeForm = document.getElementsByName("resetCodeForm")[0];

document.body.addEventListener("htmx:afterSwap", async function (event) {
  if (event.detail.xhr.status == 200) {
    await delay(300);
    spinnerDiv.style.display = "none";
    msgDiv.style.display = "block";
  }
  if (event.target.textContent.trim() == "Code Sent!") {
    await delay(1000);
    resetForm.style.display = "none";
    resetCodeForm.style.display = "block";
  }
});

document.body.addEventListener("htmx:beforeSend", function () {
  spinnerDiv.style.display = "block";
  msgDiv.style.display = "none";
});

window.onload = function () {
  document.getElementsByName("resetForm")[0].reset();
};
