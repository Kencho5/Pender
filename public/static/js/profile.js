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

var posts = htmx.findAll(".post");

for (const post of posts) {
  var deleteIcon = document.createElement("img");
  deleteIcon.src = "/assets/icons/delete.svg";
  deleteIcon.classList.add("delete-icon");
  deleteIcon.alt = "Delete";
  deleteIcon.setAttribute(
    "hx-on:mousedown",
    "htmx.find('#confirmModal').classList.toggle('show-confirm')",
  );

  post.parentNode.appendChild(deleteIcon);
}
