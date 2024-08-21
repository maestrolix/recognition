var modal = document.getElementById('myModal');
var modalImg = document.getElementById("img01");

function zoom_img(id_img) {
    console.log(id_img);
    var img = document.getElementById("myImg".concat('', id_img));
    modal.style.display = "block";
    modalImg.src = img.src;
    modalImg.alt = img.alt;
    modalImg.style.opacity = 1;
    modalImg.style.zIndex = 2;
}


modal.onclick = function() {
    img01.className += " out";
    setTimeout(function() {
        modal.style.display = "none";
        img01.className = "modal-content";
    }, 400);
} 
