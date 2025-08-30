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
* Trying to add an user to the database results in a panic, which isn't sent to the client

# 2025-03-05 - Fixing user registration
- Fixed, needed some default values
- User insertion works

# 2025-05-17 - fixing user routes & start work on decks
- Fixed get user that does not exist, and return result for user that do exist

# 2025-05-21 - turbofish, from-into and decks
- rewrote some conversion functions into "into/from" impl

# 2025-05-(23-24) - diesel schemas

# 2025-06-01 - Cont'd
- Using nanoid to generate short_ids for the decks.
- wrote short_id generation RESTful extra service

# 2025-06-21 - plugging in the short_id generator service
- plugged with reqwest the short generator id external service

# 2025-06-28 - writing the deck repository
- considerations of soft deletes and cascading
- what happens when restoring a card position-wise
- auditability wiuth audit/history tables
- factorizing access to the database

# 2025-06-29 - Fixing decks
- Factorized code
- Created models, service and repository
- fetch sid from service now works
- Can now create a deck

# 2025-07-21 - Shortid
- Added default values for config, making a default .env useless
- made the Dockerfile for the short_id_gen_api project then added it as part of the docker compose
- started revamping error handling system to make it easier to handle

# 2025-08-03 - error revamped
- revamped the system for error management

# 2025-08-05 - cont'd

# 2025-08-17 - cont'd
- finished writing ReturnableError.
- started to right more specific implementations of ResourceManagementError for UserError

# 2025-08-21 - cont'd
- finished implementing new error system for UserError
- start implementing it in the users/ folder
    - done
- start implementing it in the decks/ folder + create DeckError

# 2025-08-22 - cont'd
- continue
- also separating domain errors from app's (i.e. service.rs don't return AppError but specific domain errors. handlers.rs handle the conversion to AppError)

# 2025-08-29 - cont'd
- error management implemented for decks
- error management implemented for users
