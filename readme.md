### Simple Api written in Rust using Rocket framework

<div style="display: flex; align-items: center; justify-content: center; padding 40px">
    <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" wifth="100%" height="100px" />
</div>

### How to run
- Clone the repository
- Run `cargo run` in the root directory of the project
- The server will start at `http://localhost:8000`

### Endpoints
- `GET /` - Returns a simple message
- `GET /create` - Create a new user
- `GET /users` - Returns a list of all users
- `GET /users/{id}` - Returns a user by id
- `GET /users/{id}/delete` - Deletes a user by id

### Steps
- [x] Create a new project
- [x] Add Rocket to the project
- [x] Create a simple endpoint
- [x] Create a user model
- [x] Creating an endpoint to create a user
- [x] Creating an endpoint to get all users
- [ ] Creating an endpoint to get a user by id
- [ ] Creating an endpoint to delete a user by id
- [ ] Creating an endpoint to update a user by id 