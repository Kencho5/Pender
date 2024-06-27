function tabManager() {
  return {
    activeTab: localStorage.getItem("activeTab") || "details",

    setActiveTab(tab) {
      this.activeTab = tab;
      localStorage.setItem("activeTab", tab);
    },

    isActiveTab(tab) {
      return this.activeTab === tab;
    },
  };
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
