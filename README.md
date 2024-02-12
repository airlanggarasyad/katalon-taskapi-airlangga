# API Testing Documentation

This document provides instructions and guidelines for testing the JSONPlaceholder API using various HTTP methods such as GET, DELETE, and POST.

## Endpoints

The API endpoints provided by JSONPlaceholder are:

1. **Get User by ID**
   - Endpoint: `GET https://jsonplaceholder.typicode.com/users/{id}`
   - Example: `GET https://jsonplaceholder.typicode.com/users/9`

2. **Get Post by ID**
   - Endpoint: `GET https://jsonplaceholder.typicode.com/posts/{id}`
   - Example: `GET https://jsonplaceholder.typicode.com/posts/100`

3. **Delete User by ID**
   - Endpoint: `DELETE https://jsonplaceholder.typicode.com/users/{id}`
   - Example: `DELETE https://jsonplaceholder.typicode.com/users/9`

4. **Delete Post by ID**
   - Endpoint: `DELETE https://jsonplaceholder.typicode.com/posts/{id}`
   - Example: `DELETE https://jsonplaceholder.typicode.com/posts/100`

5. **Create User**
   - Endpoint: `POST https://jsonplaceholder.typicode.com/users`
   - Example Body:
     ```json
     {
       "name": "Jalu Potter",
       "username": "JalPot",
       "email": "jalupotter@bri.co.id",
       "address": {
         "street": "Kulas Light",
         "suite": "Apt. 556",
         "city": "Gwenborough",
         "zipcode": "92998-3874",
         "geo": {
           "lat": "-37.3159",
           "lng": "81.1496"
         }
       },
       "phone": "1-770-736-8031 x56442",
       "website": "hildegard.org",
       "company": {
         "name": "Romaguera-Crona",
         "catchPhrase": "Multi-layered client-server neural-net",
         "bs": "harness real-time e-markets"
       }
     }
     ```

6. **Create Post**
   - Endpoint: `POST https://jsonplaceholder.typicode.com/posts`
   - Example Body:
     ```json
     {
       "title": "Jalu Potter",
       "body": "In the quaint village of Mistwood, young Jalu discovers he is a descendant of a legendary wizarding lineage and must embark on a perilous journey to retrieve the Enchanted Wand before it falls into the hands of dark forces threatening to plunge the magical realm into chaos.",
       "userId": 10
     }
     ```

## Test Cases

Below are the test cases for each endpoint:

1. **Get User by ID**
   - Send a GET request to `https://jsonplaceholder.typicode.com/users/9`
   - Expected: Retrieve user information with ID 9.
   - Response Code: `200 OK`
   - Response Body
     ```json
     {
        "id": 9,
        "name": "Glenna Reichert",
        "username": "Delphine",
        "email": "Chaim_McDermott@dana.io",
        "address": {
          "street": "Dayna Park",
          "suite": "Suite 449",
          "city": "Bartholomebury",
          "zipcode": "76495-3109",
          "geo": {
            "lat": "24.6463",
            "lng": "-168.8889"
          }
        },
        "phone": "(775)976-6794 x41206",
        "website": "conrad.com",
        "company": {
          "name": "Yost and Sons",
          "catchPhrase": "Switchable contextually-based project",
          "bs": "aggregate real-time technologies"
        }
      }
     ```

2. **Get Post by ID**
   - Send a GET request to `https://jsonplaceholder.typicode.com/posts/100`
   - Expected: Retrieve post information with ID 100.
   - Response Code: `200 OK`
   - Response Body
     ```json
     {
        "userId": 10,
        "id": 100,
        "title": "at nam consequatur ea labore ea harum",
        "body": "cupiditate quo est a modi nesciunt soluta\nipsa voluptas error itaque dicta in\nautem qui minus magnam et distinctio eum\naccusamus ratione error aut"
      }
     ```

3. **Delete User by ID**
   - Send a DELETE request to `https://jsonplaceholder.typicode.com/users/9`
   - Expected: Delete user with ID 9.
   - Response Code: `200 OK`

4. **Delete Post by ID**
   - Send a DELETE request to `https://jsonplaceholder.typicode.com/posts/100`
   - Expected: Delete post with ID 100.
   - Response Code: `200 OK`

5. **Create User**
   - Send a POST request to `https://jsonplaceholder.typicode.com/users` with the provided user data.
   - Expected: Create a new user with the provided information.
   - Response Code: `201 Created`
   - Response Body
    ```json
    {
      "name": "Jalu Potter",
      "username": "JalPot",
      "email": "jalupotter@bri.co.id",
      "address": {
        "street": "Kulas Light",
        "suite": "Apt. 556",
        "city": "Gwenborough",
        "zipcode": "92998-3874",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        }
      },
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "company": {
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time e-markets"
      },
      "id": 11
    }
    ```

6. **Create Post**
   - Send a POST request to `https://jsonplaceholder.typicode.com/posts` with the provided post data.
   - Expected: Create a new post with the provided information.
   - Response Code: `201 Created`
   - Response Body
     ```json
     {
        "title": "Jalu Potter",
        "body": "In the quaint village of Mistwood, young Jalu discovers he is a descendant of a legendary wizarding lineage and must embark on a perilous journey to retrieve the Enchanted Wand before it falls into the hands of dark forces threatening to plunge the magical realm into chaos.",
        "userId": 10,
        "id": 101
      }
     ```

## Testing Tools

All test cases are conducted using Katalon Studio.
