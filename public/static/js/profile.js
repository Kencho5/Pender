function changeTab(clickedElement, targetDivId) {
  const selectorItems = document.querySelectorAll(".selector-item");
  const detailsDiv = document.querySelector(".details-div");
  const postsDiv = document.querySelector(".posts-div");

  selectorItems.forEach((item) => item.classList.remove("active-selector"));

  clickedElement.classList.add("active-selector");

  const isPostsTab = targetDivId === "posts";
  postsDiv.classList.toggle("hide-tab", !isPostsTab);
  detailsDiv.classList.toggle("hide-tab", isPostsTab);
}
