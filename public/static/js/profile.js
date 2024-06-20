var profileUrl = new URLSearchParams(window.location.search);
var tab = profileUrl.get("tab") || "profile-selector";
var target = tab == "profile-selector" ? "details" : "posts";
changeTab(htmx.find(`.${tab}`), target);

function changeTab(clickedElement, targetDivId) {
  const selectorItems = document.querySelectorAll(".selector-item");
  const detailsDiv = document.querySelector(".details-div");
  const postsDiv = document.querySelector(".posts-div");

  selectorItems.forEach((item) => item.classList.remove("active-selector"));

  clickedElement.classList.add("active-selector");

  const isPostsTab = targetDivId === "posts";
  postsDiv.classList.toggle("hide-tab", !isPostsTab);
  detailsDiv.classList.toggle("hide-tab", isPostsTab);

  profileUrl.set("tab", isPostsTab ? "posts-selector" : "profile-selector");
  window.history.replaceState(
    {},
    "",
    `${window.location.pathname}?${profileUrl.toString()}`,
  );
}

var posts = htmx.findAll(".post");
for (const post of posts) {
  var deleteIcon = document.createElement("img");
  deleteIcon.src = "/assets/icons/delete.svg";
  deleteIcon.classList.add("delete-icon");
  deleteIcon.alt = "Delete";
  deleteIcon.setAttribute(
    "onmousedown",
    `const modal = htmx.find('#confirmModal');
    modal.classList.toggle('show-confirm');
    modal.setAttribute('postId', ${post.id});`,
  );
  post.parentNode.appendChild(deleteIcon);
}

function deletePost() {
  const postID = htmx.find(".confirm-modal").getAttribute("postid");

  htmx.find(".deleteBtn").disabled = true;
  htmx.ajax("POST", `/api/delete-post/${postID}`, {});
}
