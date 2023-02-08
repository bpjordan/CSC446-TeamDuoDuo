var parsedUrl = new URL(window.location.href);

// Function that logs the user in
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
        // If no errors, convert response into json
        .then(res => {
            if (res.ok) {
                return res.json();

            }
            else { throw Error(res.status) }
        })

        // Display MFA modal to user
        // Also save MFA token from response in modal form
        .then((data) => {
            document.getElementById("mfa-modal").style.display = "block";
            document.getElementById("auth_token").value = data.mfa_token;
        })

        // Display errors to user
        .catch((error) => {
            if (error.message == 401) {
                document.getElementById("error").textContent = "Incorrect username or password, please try again.";
            }
            else {
                console.log(error);
            }
        });
};

// Function that checks user's mfa code
function mfa(e) {
    // Stop form submission
    e.preventDefault();

    // Get form using id
    var form = document.getElementById("mfa-form");

    // Authenticate user using form data
    fetch("http://" + parsedUrl.host + "/api/mfa", {
        method: "POST",
        body: new URLSearchParams(new FormData(form)),
        mode: "no-cors"
    })

        // Check for errors in request
        // If no errors, navigate to main page
        .then(res => {
            if (res.ok) {
                location.href = "/index.html";
            }
            else { throw Error(res.status) }
        })

        // Display errors to user
        .catch((error) => {
            if (error.message == 401) {
                document.getElementById("mfa-error").textContent = "Incorrect code, please try again.";
            }
            else if (error.message == 400) {
                document.getElementById("mfa-error").textContent = "Token has expired, please try logging in again.";
            }
            else if (error.message == 422) {
                document.getElementById("mfa-error").textContent = "Incorrect code length, please try again.";
            }
            else {
                document.getElementById("mfa-error").textContent = error.message + " Error";
            }
        });
}

// Function that checks user's mfa code
function getComments() {

    // Get comments
    fetch("http://" + parsedUrl.host + "/api/blog/query", {
        method: "GET",
        mode: "no-cors"
    })

        // Check for errors in request
        // If no errors, convert response to json
        .then(res => {
            if (res.ok) {
                return res.json();
            }
            else { throw Error(res.status) }
        })

        // Add comments to page
        .then((data) => {
            var existingComments = document.getElementById("blog-comments").children;

            // Update comments only if there is a new comment or no comments
            if (existingComments.length < data.length) {

                // Add each comment to page
                var comments = ""
                for (let index = 0; index < data.length; index++) {
                    const content = data[index]['content'];
                    comments += "<div class='blog-comment'><img src='https://cdn-icons-png.flaticon.com/512/287/287226.png' style='width: 50px; height: 50px;' /><div><div style='font-weight: bold; margin-bottom: 10px;'>Anonymous</div><div>" + content + "</div></div></div>"
                }
                document.getElementById("blog-comments").innerHTML = comments
            }
        })

        // Display errors to user
        .catch((error) => {
            console.log(error);
        });
}

// Function that checks user's mfa code
function postComment(e) {
    // Stop form submission
    e.preventDefault();

    // Get form using id
    var form = document.getElementById("comment-form");
    var comment = document.getElementById("comment-form").children[0].value;

    // If the user did not type anything, display error
    if (comment.length == 0) {
        document.getElementById("comment-error").innerHTML = "Your comment cannot be empty."
        return;
    }

    // Send form data
    fetch("http://" + parsedUrl.host + "/api/blog/add_comment", {
        method: "PUT",
        body: new URLSearchParams(new FormData(form))
    })

        // Check for errors in request
        // If no errors, update comments
        .then(res => {
            if (res.ok) {
                // Update list of comments
                getComments()

                // Clear comment box and any error messages
                document.getElementById("comment-content").value = ""
                document.getElementById("comment-error").innerHTML = ""
            }
            else { throw Error(res.status) }
        })

        // Display errors to user
        .catch((error) => {
            console.log(error);
        });
}

// Function that queries the database
function query(type) {

    // Request data using session token
    fetch("http://" + parsedUrl.host + "/api/query" + type, {
        method: "GET",
        mode: "no-cors"
    })

        // Check for errors in request
        .then(res => {
            if (res.ok) {

                // If logs, convert response into text
                if (type == "/access_logs") {
                    return res.text();
                }

                // Otherwise convert response into json
                else {
                    return res.json();
                }
            }
            else { throw Error(res.status); }
        })

        // Check type to determine data logic
        .then((data) => {
            // If user request, add sprite to box
            if (type == "/user") {
                // Create user sprite
                var image = '<img \
                    class="sprite" \
                    src="' + data['sprite'] + '" \
                    data-name="' + data['username'] + '" \
                    data-role="' + data['role'] + '" \
                    data-image="' + data['image'] + '" \
                    onclick="updateUserData(event)"/>';

                // Add user sprite to box
                document.getElementById('box-area-users').innerHTML = image;

                // Click sprite to display user data
                document.getElementById('box-area-users').firstChild.click();

                // Update page to display user's name
                document.getElementById("user-pc").innerHTML = data['username'].toUpperCase() + "'s PC";
            }

            // If users request, add sprites to box
            else if (type == "/users") {

                // Create user sprites
                var images = '';
                for (var i = 0; i < data.length; i++) {
                    images += '<img \
                    class="sprite" \
                    src="' + data[i]['sprite'] + '" \
                    data-name="' + data[i]['username'] + '" \
                    data-role="' + data[i]['role'] + '" \
                    data-image="' + data[i]['image'] + '" \
                    onclick="updateUserData(event)"/>';
                }

                // Add user sprites to box
                document.getElementById('box-area-users').innerHTML = images;

                // Click sprite to display user data
                document.getElementById('box-area-users').firstChild.click();
            }

            // If user pokemon request, add sprite to box
            else if (type == "/user_pokemon") {

                // Create pokemon sprite
                var image = '<img \
                    class="sprite" \
                    src="' + data[0]['sprite'] + '" \
                    data-name="' + data[0]['name'] + '" \
                    data-type="' + data[0]['type'] + '" \
                    data-image="' + data[0]['image'] + '" \
                    onclick="updatePokemonData(event)"/>';

                // Add pokemon sprite to box
                document.getElementById('box-area-pokemon').innerHTML = image;

                // Click sprite to display pokemon data
                document.getElementById('box-area-pokemon').firstChild.click();
            }

            // If pokemon request, add sprites to box
            else if (type == "/pokemon") {

                // Create pokemon sprites
                var images = '';
                for (var i = 0; i < data.length; i++) {
                    images += '<img \
                    class="sprite" \
                    src="' + data[i]['sprite'] + '" \
                    data-name="' + data[i]['name'] + '" \
                    data-type="' + data[i]['type'] + '" \
                    data-image="' + data[i]['image'] + '" \
                    onclick="updatePokemonData(event)"/>';
                }

                // Add pokemon sprites to box
                document.getElementById('box-area-pokemon').innerHTML = images;

                // Click sprite to display pokemon data
                document.getElementById('box-area-pokemon').firstChild.click();
            }

            // If logs request, add log data to box
            else if (type == "/access_logs") {

                // Add log data to box
                document.getElementById('box-area-logs').innerHTML = data;

                // Display log data
                document.getElementById("pokemon").style = "display:none;"
                document.getElementById("users").style = "display:none;"
                document.getElementById("logs").style = "display:flex;"
            }
        })

        // Log request errors
        .catch((error) => {

            // If token expired, alert and log out user
            if (error.message == 401) {
                alert("Session Expired: You will be redirected to the login page.");
                document.cookie = "session_token=invalid";
                location.href = "/";
            }

            // If insufficient permission, alert user
            if (error.message == 403) {
                alert("Insufficient Permissions: You do not have permission to view this information.");
            }
            else {
                console.log(error);
            }
        })
}

// Function that gets the data for each query type
function getData(type) {

    if (type == "/user") {

        // Check if data has not been set
        if (document.getElementById("box-area-users").innerHTML == "") {

            // GET user data
            query(type);
        }

        // Display user data
        document.getElementById("error").textContent = "";
        document.getElementById("pokemon").style = "display:none;"
        document.getElementById("users").style = "display:flex;"
        document.getElementById("logs").style = "display:none;"

    }
    else if (type == "/users") {

        // GET all user data
        query(type);

    }
    else if (type == "/user_pokemon") {

        // Check if data has not been set
        if (document.getElementById("box-area-pokemon").innerHTML == "") {

            // GET user pokemon data
            query(type);
        }

        // Display user data
        document.getElementById("error").textContent = "";
        document.getElementById("pokemon").style = "display:flex;"
        document.getElementById("users").style = "display:none;"
        document.getElementById("logs").style = "display:none;"
    }
    else if (type == "/pokemon") {

        // GET all pokemon data
        query(type);

    }
    else if (type == "/access_logs") {

        // GET log data
        query(type);

    }

};

// Function that updates the user data when a sprite is selected
function updateUserData(e) {

    // Update user data to display clicked sprite information
    document.getElementById("user-data-name").textContent = e.target.getAttribute('data-name').toUpperCase();
    document.getElementById("user-data-role").textContent = e.target.getAttribute('data-role').toUpperCase();
    document.getElementById("user-data-image").src = e.target.getAttribute('data-image');

    // Change color of role background and image border based on role
    role = e.target.getAttribute('data-role');
    if (role == "Trainer") {
        document.getElementById("user-data-role").style.backgroundColor = '#ff0000';
        document.getElementById("user-data-role").style.border = 'solid #a10000 2px';
        document.getElementById("user-data-border").style.borderColor = '#ff0000';
    }
    else if (role == "Professor") {
        document.getElementById("user-data-role").style.backgroundColor = '#D0D1CD';
        document.getElementById("user-data-role").style.border = 'solid #818181 2px';
        document.getElementById("user-data-border").style.borderColor = '#D0D1CD';
    }
    else {
        document.getElementById("user-data-role").style.backgroundColor = '#FFDE00';
        document.getElementById("user-data-role").style.border = 'solid #a99200 2px';
        document.getElementById("user-data-border").style.borderColor = '#FFDE00';
    }
}

// Function that updates the pokemon data when a sprite is selected
function updatePokemonData(e) {

    // Update pokemon data to display clicked sprite information
    document.getElementById("pokemon-data-name").textContent = e.target.getAttribute('data-name').toUpperCase();
    document.getElementById("pokemon-data-type").textContent = e.target.getAttribute('data-type').toUpperCase();
    document.getElementById("pokemon-data-image").src = e.target.getAttribute('data-image');

    // Change color of type background and image border based on type
    type = e.target.getAttribute('data-type');
    if (type == "fire") {
        backgroundColor = '#ff4422';
        borderColor = '#a10000';
    }
    else if (type == "grass") {
        backgroundColor = '#77cc55';
        borderColor = '#4d8537';
    }
    else if (type == "water") {
        backgroundColor = '#3399ff';
        borderColor = '#246fba';
    }
    else if (type == "electric") {
        backgroundColor = '#ffcc33';
        borderColor = '#c59d24';
    }
    else if (type == "rock") {
        backgroundColor = '#bbaa66';
        borderColor = '#938651';
    }
    else {
        backgroundColor = '#aaaa99'
        borderColor = '#77776a'
    }
    document.getElementById("pokemon-data-type").style.backgroundColor = backgroundColor;
    document.getElementById("pokemon-data-type").style.border = 'solid ' + borderColor + ' 2px';
    document.getElementById("pokemon-data-border").style.borderColor = backgroundColor;
}

// Function that logs the user out
function logout() {
    document.cookie = "session_token=invalid";
    location.href = "/";
}