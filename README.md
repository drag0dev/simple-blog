# SimpleBlog
This project is a simple web application for posting and reading blog posts.

## Features
- **Create Blogposts**: users can create blog posts consisting of text, publication date, an optional image, a user name, and an optional avatar
- **View Blogposts**: the blog post feed displays all posts, including text, date, optional image, user name, and optional user avatar

## Prerequisites
- Docker
- Docker Compose

## Installation
```
git clone https://github.com/drag0dev/simple-blog.git
cd simple-blog
docker-compose up -d
```
Once the container is running, the application will be available at http://localhost:8082/home.

## API Endpoints
- POST /api/v1/blogpost - create a new blog post, accepts a multipart form
- GET /api/v1/blogpost?page=n - fetch the nth page of the feed, where each page has five posts
- GET  /api/v1/image/{uuid} - fetch the image with the given uuid

## Notes
- Images uploaded for the blog posts and user avatars will be saved in the images directory on the server
- Both post image and the avatar have to be a PNG image and not larger than 2MB
- User's avatar will be downloaded and stored from the URL provided during blog post creation