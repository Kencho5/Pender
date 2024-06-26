var inputs = document.querySelectorAll('input[type="submit"]');

document.body.addEventListener("htmx:beforeSend", function () {
  inputs.forEach((input) => {
    input.disabled = true;
    input.style.backgroundColor = "#E78882";
  });
});

document.body.addEventListener("htmx:afterSwap", function (evt) {
  inputs.forEach((input) => {
    input.disabled = false;
    input.style.backgroundColor = "#D9625A";
  });

  const xhr = evt.detail.xhr;
  const response = xhr.responseText;

  if (response.includes("Code Sent!")) {
    const resetForm = document.querySelector('form[name="resetForm"]');
    const resetCodeForm = document.querySelector('form[name="resetCodeForm"]');

    resetForm.style.display = "none";
    resetCodeForm.style.display = "block";
    resetCodeForm.reset();
  }
});
