fetch("https://status.cafe/users/nonk/status.json")
    .then((r) => r.json())
    .then(cafe);

function cafe(response) {
    const usernameLink = document.createElement("a");
    usernameLink.href = "https://status.cafe/users/nonk";
    usernameLink.target = "_blank";
    usernameLink.textContent = response.author;

    const username = document.getElementById("statuscafe-username");
    username.appendChild(usernameLink);

    const content = document.getElementById("statuscafe-content");
    if (!response.content.length) {
        content.textContent = "No status yet.";
        return;
    }

    const marginalia = document.createTextNode(
        " " + response.face + " " + response.timeAgo
    );
    username.appendChild(marginalia);

    content.innerHTML = response.content;
}
