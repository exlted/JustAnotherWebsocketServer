// When a message is sent, display it
document.addEventListener("messageRecieved", function(e) {
    let div = document.createElement("div");
    div.innerHTML = e.detail;
    document.body.appendChild(div);
})
