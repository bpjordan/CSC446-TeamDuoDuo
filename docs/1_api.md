
# API Endpoints

## /

Returns the web app's frontend.

## /api

Root directory for all non-html API endpoints.  

Note: All APIs will respond with `500 Internal Server Error` if any error occurs.  

### /api/query (GET)

Queries the database server for all entries in the `users` table.  
Requires a header with the token provided by `/api/login`.
Responds with `200 OK` and a JSON string representing the `users` table.
Request must include a valid authorization token in its cookies,
otherwise endpoint returns `401 Unauthorized`

### /api/login (POST)

Checks if a user exists in the database with the correct username & password.  
If login is correct, returns `200 OK` status code and an authorization cookie is added.
If login is incorrect, returns `401 Unauthorized` status code.  

Request body should include HTTP form with parameters:

- `username`: username to check
- `password`: password to check, as plaintext
