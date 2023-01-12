var parsedUrl = new URL(window.location.href);

function query() {

    // Request data using session token
    fetch("http://" + parsedUrl.host + "/api/query", {
        method: "GET",
        mode: "no-cors"
    })

    // Check for errors in request
    .then (res => {
        if (res.ok) {return res.text()}
        else {throw Error(res.status)}
    })

    // Display data to user
    .then((data) => {
        document.getElementById("response").innerHTML = data;
    })

    // Log request errors
    .catch((error) => {
        if (error.message == 401){
            alert("Session Expired: You will be redirected to the login page.")
            document.cookie = ""
            location.href = "/"
        }
    })
}

function login(e) {

    // Stop form submission
    e.preventDefault();

    // Get form using id
    var form = document.getElementById("loginForm");

    // Authenticate user using form data
    fetch("http://" + parsedUrl.host + "/api/login", {
        method: "POST",
        body: new URLSearchParams(new FormData(form)),
        mode: "no-cors"
    })

    // Check for errors in request
    .then (res => {
        if (res.ok) {return res.text()}
        else {throw Error(res.status)}
    })

    // Save session token and naviagte to main page
    .then(body => {
        document.cookie = JSON.parse(body).token;
        location.href = "/index.html";
    })

    // Display errors to user
    .catch((error) => {
        if (error.message == 401) {
            document.getElementById("error").textContent = "Incorrect username or password, please try again.";
        }
    });
}