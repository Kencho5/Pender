function switchTab(toHide, toShow) {
  document.getElementsByClassName(toHide)[0].style.display = "none";
  document.getElementsByClassName(toShow)[0].style.display = "block";
}
