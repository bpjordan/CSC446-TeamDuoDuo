var parsedUrl = new URL(window.location.href);

function query() {

    // Request data using session token
    fetch("http://" + parsedUrl.host + "/api/query", {
        method: "GET",
        mode: "no-cors"
    })

        // Check for errors in request
        .then(res => {
            if (res.ok) { return res.json(); }
            else { throw Error(res.status); }
        })

        // Create user sprites
        .then((data) => {
            var images = '';
            for (var i = 0; i < data.length; i++) {
                images += '<img \
                class="sprite" \
                src="https://media.pokemoncentral.it/wiki/c/c9/RFVF_Rosso.png" \
                data-name="' + data[i]['username'] + '" \
                data-role="' + data[i]['role'] + '" \
                data-image="https://media.pokemoncentral.it/wiki/b/be/RossoRFVF.png" \
                onclick="updateUserData(event)"/>';
            }
            document.getElementById('box-area-users').innerHTML = images;
        })

        // Log request errors
        .catch((error) => {
            if (error.message == 401) {
                alert("Session Expired: You will be redirected to the login page.");
                document.cookie = "session=invalid";
                location.href = "/";
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
        // If no errors, navigate to main page
        .then(res => {
            if (res.ok) {
                document.cookie = "session=valid"
                location.href = "/index.html";
            }
            else { throw Error(res.status) }
        })

        // Display errors to user
        .catch((error) => {
            if (error.message == 401) {
                document.getElementById("error").textContent = "Incorrect username or password, please try again.";
            }
        });
};

function getUsers() {

    // Check if data is missing
    if (document.getElementById("box-area-users").textContent != "") {

        // GET user data
        query();
    }
    // If authorized, display user data
    document.getElementById("error").textContent = "";
    document.getElementById("pokemon").style = "display:none;"
    document.getElementById("users").style = "display:flex;"
};

function getLogs() {
    // Display error message
    document.getElementById("error").textContent = "You are not authorized to view logs.";

};

function getPokemon() {
    // Display pokemon data
    document.getElementById("error").textContent = "";
    document.getElementById("pokemon").style = "display:flex;"
    document.getElementById("users").style = "display:none;"
};

function logout() {
    document.cookie = "session=invalid";
    location.href = "/";
}

function updateUserData(e) {
    document.getElementById("user-data-name").textContent = e.target.getAttribute('data-name').toUpperCase();
    document.getElementById("user-data-role").textContent = e.target.getAttribute('data-role').toUpperCase();
    document.getElementById("user-data-image").src = e.target.getAttribute('data-image');
    roleColor(e.target.getAttribute('data-role'));
}

function updatePokemonData(e) {
    document.getElementById("pokemon-data-name").textContent = e.target.getAttribute('data-name').toUpperCase();
    document.getElementById("pokemon-data-type").textContent = e.target.getAttribute('data-type').toUpperCase();
    document.getElementById("pokemon-data-image").src = e.target.getAttribute('data-image');
    typeColor(e.target.getAttribute('data-type'));
}

function roleColor(role) {
    if (role == "trainer") {
        document.getElementById("user-data-role").style.backgroundColor = '#ff0000';
        document.getElementById("user-data-border").style.borderColor = '#ff0000';
    }
    else if (role == "professor") {
        document.getElementById("user-data-role").style.backgroundColor = '#D0D1CD';
        document.getElementById("user-data-border").style.borderColor = '#D0D1CD';
    }
    else {
        document.getElementById("user-data-role").style.backgroundColor = '#FFDE00';
        document.getElementById("user-data-border").style.borderColor = '#FFDE00';
    }
}

function typeColor(type) {
    console.log(type);
    if (type == "fire") {
        document.getElementById("pokemon-data-type").style.backgroundColor = '#ff4422';
        document.getElementById("pokemon-data-border").style.borderColor = '#ff4422';
    }
    else if (type == "grass") {
        document.getElementById("pokemon-data-type").style.backgroundColor = '#77cc55';
        document.getElementById("pokemon-data-border").style.borderColor = '#77cc55';
    }
    else if (type == "water") {
        document.getElementById("pokemon-data-type").style.backgroundColor = '#3399ff';
        document.getElementById("pokemon-data-border").style.borderColor = '#3399ff';
    }
    else if (type == "electric") {
        document.getElementById("pokemon-data-type").style.backgroundColor = '#ffcc33';
        document.getElementById("pokemon-data-border").style.borderColor = '#ffcc33';
    }
    else if (type == "rock") {
        document.getElementById("pokemon-data-type").style.backgroundColor = '#bbaa66';
        document.getElementById("pokemon-data-border").style.borderColor = '#bbaa66';
    }
    else {
        document.getElementById("pokemon-data-type").style.backgroundColor = '#aaaa99';
        document.getElementById("pokemon-data-border").style.borderColor = '#aaaa99';
    }
}