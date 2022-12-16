
# Building & Running

A `docker-compose.yaml` file is provided to easily build and run all components of the service.  

## Building with `docker compose`

To build the service, enter the project's root directory and run `docker compose build`.  
Once the service images are built, start the service with `docker compose up`.  
Stop the service with `docker compose down`.  

## Creating documentation

A separate docker service is provided to convert the documentation files in `docs/` to this PDF document.  
Run this service with `docker compose run pandoc` from the root project directory.
