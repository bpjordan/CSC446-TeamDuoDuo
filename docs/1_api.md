
# API

## Endpoints

### /
Returns the web app's frontend.

### /api
Root directory for all API endpoints

#### /api/query
Queries the database server for all entries in the `users` table.
May either respond with the database's contents or an HTTP 500 error.