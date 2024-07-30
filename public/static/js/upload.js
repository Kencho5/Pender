var form = {
  user_id: null,
  user_name: null,
  animal: null,
  breed: null,
  post_type: null,
  price: null,
  age_type: null,
  age: null,
  sex: null,
  phone: null,
  city: null,
  description: null,
  photos: [],
};

// var form = {
//   user_id: "asd",
//   user_name: "asd",
//   animal: "asd",
//   breed: "asd",
//   post_type: "asd",
//   price: null,
//   age_type: "asd",
//   age: "123",
//   sex: "asd",
//   phone: "123",
//   city: "asd",
//   description: "asd",
//   photos: [],
// };
var step = 1;
var stepsArr = {
  2: [1, 2],
  5: [3],
  6: [3],
};

function setData(key, value) {
  form[key] = value;
}

function showData() {
  if (Object.keys(form).length == 0) return;

  for (const [key, value] of Object.entries(form)) {
    const element = htmx.find(`[input-data="${value}"]`);
    const inputs = htmx.findAll(`[input-data="${key}"]`);

    if (!inputs) continue;
    if (element) element.classList.add("active-chip");

    inputs.forEach((input) => {
      if (input.type == "file" && form.photos) {
        const dataTransfer = new DataTransfer();
        value.forEach((el) => dataTransfer.items.add(el.file));

        input.files = dataTransfer.files;
        return;
      }

      input.value = value;
    });
  }
}

document.body.addEventListener("htmx:afterSettle", function (event) {
  if (!event.target.className.includes("info")) return;
  if (step == 7) upload();

  showData();
});

function changeStep(dir) {
  if (dir == "next") step++;
  else step--;
  step = Math.max(1, Math.min(step, 7));
  window.scrollTo({ top: 0, behavior: "smooth" });

  if (!stepsArr.hasOwnProperty(step)) return;

  for (const x of stepsArr[step]) {
    const icon = htmx.find(`#step${x}`);
    const prevStep = Math.max(1, icon.id.split("step")[1] - 1);
    const prevIcon = htmx.find(`#step${prevStep}`);

    prevIcon.children[0].style.opacity = "1";
    prevIcon.children[1].classList.add("hidden");
    prevIcon.children[2].classList.remove("hidden");
    icon.parentNode.classList.add("steps__active");

    if (step == 7) {
      icon.children[0].classList.add("hidden");
      icon.children[1].classList.remove("hidden");
    }
  }
}

function validateStep(dir) {
  let valid = true;
  const inputs = htmx.findAll("input");
  const description = htmx.find("textarea");

  inputs.forEach((input) => {
    if (!input.value) valid = false;
  });
  if (description && !description.value) valid = false;

  if (valid) {
    htmx.trigger(".actions__next", "validated");
    changeStep(dir);
  } else {
    triggerToast("გთხოვთ შეავსოთ ყველა ველი", "#e78882", "close");
  }
}

function fileUpload() {
  return {
    images: form.photos || [],
    showInfo: form.photos.length === 0,

    handleFileChange(event) {
      const files = event.target.files;
      const hasHeif = Array.from(files).some(
        (file) => file.type === "image/heif" || file.type === "image/heic",
      );

      if (form.photos.length + files.length > 3) {
        triggerToast("აირჩიეთ მხოლოდ 3 ფოტო", "#e78882", "close");
        return;
      } else if (hasHeif) {
        triggerToast("მხოლოდ JPG ან PNG ფოტოები", "#e78882", "close");
        return;
      }
      this.showInfo = files.length === 0;

      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        this.images.push({
          url: URL.createObjectURL(file),
          file: file,
          name: file.name,
        });
      }
      this.setFormPhotos();
    },

    deleteImage(name) {
      const input = htmx.find("input");
      const files = Array.from(input.files);
      const newFileList = new DataTransfer();

      files.forEach((file) => {
        if (file.name !== name) {
          newFileList.items.add(file);
        }
      });
      input.files = newFileList.files;

      this.images = this.images.filter((image) => image.name !== name);
      this.showInfo = this.images.length === 0;

      this.setFormPhotos();
    },

    setFormPhotos() {
      form.photos = this.images;
    },
  };
}

async function compressImage(file, { quality = 1, type = file.type }) {
  const imageBitmap = await createImageBitmap(file);

  const canvas = document.createElement("canvas");
  canvas.width = imageBitmap.width;
  canvas.height = imageBitmap.height;
  const ctx = canvas.getContext("2d");
  ctx.drawImage(imageBitmap, 0, 0);

  const base64 = canvas.toDataURL(type, quality);

  return base64;
}

async function upload() {
  const photos = await Promise.all(
    form.photos.map(async (photo) => {
      return await compressImage(photo.file, {
        quality: 0.5,
        type: "image/jpeg",
      });
    }),
  );
  form.photos = photos;
  if (form.price) form.price = parseInt(form.price);

  const progressDiv = htmx.find(".progress-div");
  const xhr = new XMLHttpRequest();
  xhr.open("POST", "/upload", true);

  xhr.upload.addEventListener("progress", (event) => {
    if (event.lengthComputable) {
      let threshold = 20;
      const percent = Math.floor((event.loaded / event.total) * 100);
      const progressBar = htmx.find(".progress");
      const progressInfo = htmx.find(".progress-percentage");

      if (percent >= threshold) {
        progressDiv.style.display = "flex";
        progressBar.style.width = `${percent}%`;
        progressInfo.textContent = `${percent}%`;

        threshold += 20;
      }
    }
  });

  xhr.onload = async function () {
    progressDiv.classList.add("fade-out");
    progressDiv.addEventListener(
      "transitionend",
      function () {
        progressDiv.remove();
      },
      { once: true },
    );
    var element = htmx.find("#finishElement");
    element.classList.remove("hidden");
    element.classList.add("fade-in");
    setTimeout(function () {
      element.classList.add("show");
    }, 10);

    const data = JSON.parse(xhr.responseText);
    const { error, post_id } = data;

    if (error) {
      window.location.href = `/upload`;
      triggerToast("გთხოვთ სცადოთ ხელახლა", "#e78882", "close");
      return;
    }

    htmx.find("#postId").href = `/post/${post_id}`;
  };

  xhr.send(JSON.stringify(form));
}
