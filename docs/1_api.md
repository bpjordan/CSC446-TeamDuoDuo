
# API Endpoints

## /

Returns the web app's frontend.

## /api

Root directory for all non-html API endpoints

### /api/query (GET)

Queries the database server for all entries in the `users` table.
May either respond with the database's contents or an HTTP 500 error.

### /api/login (POST)

Checks if a user exists in the database with the correct username & password
If login is correct, returns `200 OK` status code
If login is incorrect, returns `401 Unauthorized` status code
If `password` is empty, returns `400 Bad Request` status code

HTTP Parameters:
- `username`: username to check
- `password`: password to check, as plaintext

Example: `http://api-host/login?username=user&password=pass`
