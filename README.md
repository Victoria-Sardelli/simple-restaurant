# Simple Restaurant App

## Introduction

Thank you for checking out my project!

This is a Simple Restaurant Application that handles requests to create, delete, and get menu item orders for tables at a fictional restaurant.

### Steps to run server:

(Note: this assumes that you are able to run the "cargo" command from the command line)

1. Download project files
2. From the root folder of the project, run "cargo run" in the command line

The server should now be running on your machine on port 8000.

Throughout my development process, I have found Postman to be extremely helpful in testing API endpoints, and would recommend it when experimenting with different requests.

### Experience with Rust:

1 week

---

## Database Schema

My project uses a SQLite database, utilizing the rusqlite wrapper.
A restaurant database is created upon startup, consisting of 3 tables:

| Name   | Columns                                                                                                 | Description              |
| ------ | ------------------------------------------------------------------------------------------------------- | ------------------------ |
| ITEMS  | item_id: integer (PK) <br>name: text                                                                    | Stores menu items        |
| TABLES | table_id: integer (PK)<br>seats: integer                                                                | Stores restaurant tables |
| ORDERS | order_id: integer (PK)<br>table_id: integer (FK)<br>item_id: integer (FK)<br>cook_time_minutes: integer | Stores orders            |

In dbsetup.rs, I initialize starter data for the ITEMS and TABLES tables. Ideally, this information would come from a separate source such as a configuration file.

In the current setup:

-   The ITEMS table is initialized with 4 items: "Fish", "Meat", "Spaghetti", "Bread".
    -   Each item is assigned an auto-generated ID upon insert, given in increasing order.
-   The TABLES table is initialized with 100 restaurant tables.
    -   Each restaurant table is assigned an auto-generated ID upon insert, given in increasing order.
    -   Each restaurant table is assigned a randomly-generated number of seats in the range of 1-4.

---

## API: How to Use

There are 5 REST endpoints currently available:

| Method | Path                         | Path Parameters                  | Request Body                              | Description                                                                    | Example                                                               |
| ------ | ---------------------------- | -------------------------------- | ----------------------------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| GET    | api/health                   | None                             | None                                      | Health check endpoint                                                          | http://localhost:8000/api/health                                      |
| GET    | api/orders/tables/{table_id} | table_id: id of restaurant table | None                                      | Gets all orders for table with given id                                        | http://localhost:8000/api/orders/tables/1                             |
| GET    | api/orders/{id}              | id: id of order                  | None                                      | Gets order with given id                                                       | http://localhost:8000/api/orders/1                                    |
| POST   | api/orders                   | None                             | JSON with table_id and item_id attributes | Creates new order of menu item with item_id for restaurant table with table_id | http://localhost:8000/api/orders/ <br>`{"table_id": 1, "item_id": 3}` |
| DELETE | api/orders/{id}              | id: id of order                  | None                                      | Deletes order with given id                                                    | http://localhost:8000/api/orders/1                                    |

---

## Future Improvements

If I had more time, I would have liked to complete the following improvements:

-   Add automated tests
    -   **Why**: Comprehensive automated testing is extremely important when building a system. Through different strategies such as unit testing and integration testing, we can ensure that each component fulfills its role and works as expected--both on its own, and as part of a larger system. Testing helps to not only catch bugs in new functionality before it is released, but also protect older functionality from being broken.
    -   **How**: In order to add proper tests to this system, I would research more deeply about testing strategies in Rust (e.g., what kinds of frameworks/libraries there are, how mocking is performed, how tests are usually structured and organized, etc.). I would seek information from a variety of sources, including official documentation, tutorials, and open source examples online. In the meantime, I have aimed to structure my code so that it enables testing of independent areas later on (i.e., splitting code into different files based on common responsibilities, introducing a configuration file, etc.). As I learn more about testing practices in Rust, I expect the need to further adapt my code to enable a smooth testing experience.
        -   According to my research, the convention in Rust is to write unit tests directly in the file being tested, and to write integration tests inside a tests folder outside of src. I would like to follow this guideline so that other developers can more easily navigate my code in the future.
    -   **What**: Some examples of areas that I would like to test include:
        -   API routes: endpoints with various HTTP methods and paths should be available and should invoke their respective handlers
        -   Handlers: based on the result of their invoked database operations, handlers should return appropriate JSON responses containing desired data or helpful error messages
        -   Database interactions: state of database should reflect expected outcome when performing operations with certain arguments
        -   Error handling: catching and/or prevention of errors
-   Extract more setup config and hard-coded values into a separate configuration file for easier customization and testing
    -   I moved the database file name into a configuration file so that the name could be changed without modifying the source code (thinking this may perhaps come in handy when testing database interaction). I think it would be helpful to move other static values that are required upon app startup here as well, such as menu item details and table information.
-   Set up client threads to simulate multiple concurrent requests in order to test how the application handles heavy load. In order to accomplish this, I would like to research more about working with threads in Rust.
