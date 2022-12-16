var parsedUrl = new URL(window.location.href);

function query() {
    fetch("http://" + parsedUrl.host + "/api/query", {
        method: "GET",
        mode: "no-cors",
    })
    .then((resp) => resp.text())
    .then((data) => {
        document.getElementById("response").innerHTML = data;
    })
    .catch((err) => {
        console.log(err);
    })
}

function login() {
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    fetch("http://" + parsedUrl.host + "/api/login?username=" + username + "&password=" + password, {
        method: "POST",
        mode: "no-cors"
    })
    .then((resp) => {
        resp.text();
        if(resp.status == 200) {
            location.href = "/index.html";
        }
        else if(resp.status == 401) {
            document.getElementById("error").textContent = "Incorrect username or password, please try again.";
        }
    })
    .catch((err) => {
        console.log(err);
    })
}