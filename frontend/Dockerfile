# Use the official nginx image as a base
FROM nginx:alpine

# Copy the built application from the dist/ directory to the nginx html directory
COPY dist/ /usr/share/nginx/html

# Start nginx server
CMD ["nginx", "-g", "daemon off;"]
