# Rust Crud Application using Warp and Mongo with JWT authentication

This boilerplate application offers the following endpoints, with JWT role-based validation on most of them:

| Path | Method |
|------|--------|
| /api/auth/register | POST |
| /api/auth/login | POST |
| /api/articles_home | GET |
| /api/articles | GET |
| /api/articles/{url} | GET |
| /api/articles/updateHomeView/{id} | GET |
| /api/articles | POST |
| /api/articles | PUT |
| /api/articles/{id} | DELETE |
| /api/articles/comments | POST |
| /api/articles/comments/{article_id}/{comment_id} | DELETE |
| /api/users | GET |
| /api/users/{id} | GET |
| /api/users/updateHomeView/{id} | GET |
| /api/users | POST |
| /api/users | PUT |
| /api/users/{id} | DELETE |
| /api/users/changePassword | PUT |

<br />

The **.env** file contains the mongodb connection details and encryption keys.

<br />


## Development environment setup

Requirements: rust toolchain, docker, docker-container

Create and start a mongodb container with docker:

    docker-compose up -d db mongo-express

Open the console at http://localhost:8081 on your browser and create the database **rust-crud**

<br />

## Running the Application
Run the application with the command:

    cargo run

Alternatively, you can run the command below to relaunch at any changes to the given resources:

    cargo watch -w src -w Cargo.toml -w .env -x run

<br />

## Testing

#### Register

    curl -H 'Content-Type: application/json' -d '{"name":"Test","email":"test@test.com","password":"abc123"}' http://localhost:8000/api/auth/register

#### Login

    curl -H 'Content-Type: application/json' -d '{"email":"test@test.com","password":"abc123"}' http://localhost:8000/api/auth/login

If everything is working, and you are using Linux/MacOS/Cygwin or have access to a bash, the one-liner below can be useful to parse the token from the response:

    TOKEN=$(curl -H 'Content-Type: application/json' -d '{"email":"test@test.com","password":"abc123"}' http://localhost:8000/api/auth/login | python -c 'import json,sys;print(json.load(sys.stdin)["access_token"])')
    echo $TOKEN
    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users
    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles

Querying the /users API without Admin role should result in an 401 Unauthorized error.

#### Change user role to Admin on Mongo console and login again.

<br />


### Articles API

#### Get articles

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles

#### Create article

    curl -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"title":"Test Article","url":"test","content":"Content of full article"}' http://localhost:8000/api/articles 

#### Get first article

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles/test

#### Update Article

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles | python -c 'import json,sys;print(json.load(sys.stdin)[0]["id"])')
    echo $ID
    curl -X PUT -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"id":'\"${ID}\"',"title":"Updated Test Article","url":"test","content":"Updated content of full article","in_home":true}' http://localhost:8000/api/articles

Check article after updated:

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles/test

#### Delete article

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles | python -c 'import json,sys;print(json.load(sys.stdin)[0]["id"])')
    curl -X DELETE -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles/${ID}

### Users API

##### Get users

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users

#### Create user

    curl -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"email":"TestUser","name":"test","password":"abc123","role":"User"}' http://localhost:8000/api/users 

#### Get new user

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users | python -c 'import json,sys;print(json.load(sys.stdin)[1]["id"])')
    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users/$ID

#### Update new user

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users | python -c 'import json,sys;print(json.load(sys.stdin)[1]["id"])')
    curl -X PUT -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"id":'\"${ID}\"',"email":"UpdatedTestUser","name":"test","role":"User"}' http://localhost:8000/api/users 

Get updated user field

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users | python -c 'import json,sys;print(json.load(sys.stdin)[1]["email"])'

<br />

### **Building the application**

#### Run cargo build

    cargo build --release

    