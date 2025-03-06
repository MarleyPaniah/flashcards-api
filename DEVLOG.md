# 2024-04-20 - Start of the project
* Learning how to create an HTTP API in Rust
* Choosing the framework among Actix_web, Rocket, Axum...
* Chose Axum
* Started to draft design the basic endpoints of the API
* Learning:
    * Deciding what structure to use: CQRS Command Query Responsibility Segregation, or MVC or Django
    * How to use crates
* Started to code

# 2024-04-21 - Cont'd
* Trying to nest all the routes behind the same API
* Choosing the db crate to use: diesel or sqlx

# 2024-04-28 - Cont'd
* Creating a single database with diesel
* Mixing sources to create my app, including:
https://medium.com/@qkpiot/building-a-robust-rust-backend-with-axum-diesel-postgresql-and-ddd-from-concept-to-deployment-b25cf5c65bc8

# 2024-06-25 - Cont'd
* Try to remember where I was
* Fixed issue with code postgres insert

# 2024-07-14 - Cont'd
* Try to remember where I was

# 2025-02-28 - Cont'd
* Try to remember where I was
* workout the user datamodel + database schema + auth and authorizations

# 2025-03-{01, 02} - Error handling
* Trying to add an user to the database results in a panic, which hasn't isn't sent to the client

# 2025-03-05 - Fixing user registration
- Fixed, needed some default values
- User insertion works