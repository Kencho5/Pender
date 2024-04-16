function switchTab(el) {
  const profileInfo = document.getElementsByClassName("profile-info")[0];
  const postsInfo = document.getElementsByClassName("posts-info")[0];

  document.querySelector(".active-profile-tab").classList.remove(
    "active-profile-tab",
  );
  el.classList.add("active-profile-tab");

  if (el.classList.contains("profile-tab")) {
    postsInfo.style.display = "none";
    profileInfo.style.display = "block";
  } else {
    postsInfo.style.display = "block";
    profileInfo.style.display = "none";
  }
}
