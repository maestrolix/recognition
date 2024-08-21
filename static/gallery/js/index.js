// Popup to upload new images
const importFile = document.getElementById("import_file");
const popup = document.getElementById("popup");

function showPopup() {
  importFile.style.display = "block";
  popup.style.display = "block";
}

function hidePopup() {
  importFile.style.display = "none";
  popup.style.display = "none";
}


// Preview upload image
function upload() {

    const fileUploadInput = document.querySelector('.file-uploader');

    // using index [0] to take the first file from the array
    const image = fileUploadInput.files[0];
  
    // check if the file selected is not an image file
    if (!image.type.includes('image')) {
      return alert('Only images are allowed!');
    }
  
    // check if size (in bytes) exceeds 10 MB
    if (image.size > 10_000_000) {
      return alert('Maximum upload size is 10MB!');
    }
    const fileReader = new FileReader();
    fileReader.readAsDataURL(image);
  
    fileReader.onload = (fileReaderEvent) => {
      const profilePicture = document.querySelector('.profile-picture');
      profilePicture.style.backgroundImage = `url(${fileReaderEvent.target.result})`;
    }
}
