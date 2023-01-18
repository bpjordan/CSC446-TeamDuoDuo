
# API Endpoints

## /

Returns the web app's frontend.

## /api

Root directory for all non-html API endpoints.

Note: All APIs will respond with `500 Internal Server Error` if any error occurs.

All endpoints besides `/api/login` require a valid authorization token in the request cookies.
Otherwise, endpoint will return `401 Unauthorized`

### /api/query (GET)

Returns a JSON list of `/api/query/` endpoints the current user is allowed to call.
Returned names are relative to `/api/query/`. For example: if the user is allowed to
call `/api/query/users`, the returned list will include `"users"`.

### /api/query/users, /api/query/access_logs, /api/query/pokemon (GET)

Queries the database server for all entries in the `users`, `access_logs`, or `pokemon` tables.
Responds with `200 OK` and a JSON string representing the requested table.

Includes an optional `limit=` URL parameter to limit the number of records returned

### /api/query/user, /api/query/user_pokemon (GET)

Returns information about the user making the request, or their pokemon.
Responds with `200 OK` and a JSON string

### /api/login (POST)

Checks if a user exists in the database with the correct username & password.
If login is correct, returns `200 OK` status code and an authorization cookie is added.
If login is incorrect, returns `401 Unauthorized` status code.

Request body should include HTTP form with parameters:

- `username`: username to check
- `password`: password to check, as plaintext
