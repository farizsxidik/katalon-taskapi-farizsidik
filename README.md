# JSONPlaceholder API Testing with Katalon Studio

This project documents the process of testing the JSONPlaceholder API using Katalon Studio. The aim is to validate the functionality of the API through GET, POST, and DELETE methods on selected endpoints.

## Overview

The JSONPlaceholder API offers a simulated set of CRUD operations on a blog-like platform. This project focuses on testing three key functionalities:

- Retrieving all posts (`GET /posts`)
- Creating a new post (`POST /posts`)
- Deleting a specific post (`DELETE /posts/{id}`)

## Tools Used

- **Katalon Studio**: An integrated development environment (IDE) used for automating web and API testing.

## Setup

1. **Download Katalon Studio**: Ensure you have the latest version of Katalon Studio installed. Download it from [https://www.katalon.com/](https://www.katalon.com/).
2. **Project Configuration**: Open Katalon Studio and set up a new project tailored to API testing. Name the project as per your requirements.

## Test Cases

### 1. Get Posts

- **Objective**: To retrieve a list of all posts.
- **Method**: GET
- **Endpoint**: `/posts`
- **Expected Outcome**: A successful response with status code 200 and a JSON array of posts.

### 2. Create Post

- **Objective**: To create a new post.
- **Method**: POST
- **Endpoint**: `/posts`
- **Body**:
    ```json
    {
      "title": "Test Title",
      "body": "Test Body",
      "userId": 1
    }
    ```
- **Expected Outcome**: A successful response with status code 201, including the details of the created post.

### 3. Delete Post

- **Objective**: To delete a specific post.
- **Method**: DELETE
- **Endpoint**: `/posts/{id}` (Replace `{id}` with the ID of the post to be deleted.)
- **Expected Outcome**: A successful response with status code 200, indicating the post has been deleted.

## Execution

Execute each test case from the Test Cases section in Katalon Studio by selecting the test case and clicking on the run button. Ensure you review the results in the Log Viewer to verify the test outcomes.

## Results

Document the outcomes of each test case execution, noting any discrepancies or issues encountered during testing.

## Additional Resources

For screen recordings of the test executions and any supplementary materials related to this project, please visit the following Google Drive folder:

[Testing Materials and Screen Recordings](https://drive.google.com/drive/folders/1dzYIHdaLwx10_M-OGsrvVWQoWD_kUSha?usp=sharing)

This folder includes visual documentation of the API testing process within Katalon Studio, showcasing the setup, execution, and results of each test case.

## Conclusion

This testing initiative confirms the functionality of the JSONPlaceholder API's endpoints for basic CRUD operations. The detailed testing process, along with the results and supplementary materials, are documented to facilitate understanding and further testing.
