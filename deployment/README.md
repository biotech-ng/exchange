#### Run CI job to build a Docker image

1. Navigate to the https://github.com/biotech-ng/exchange/actions/workflows/build_upload_image.yml
2. Run the `build_upload_image` job.
3. After job finished you can get the image using command in terminal: `docker pull sturivnyi/rust-wasm`.
4. Image locates here https://hub.docker.com/r/sturivnyi/rust-wasm

#### Build and run Dockerfile on a local machine

1.  Open a terminal or command prompt window and navigate to the directory where you saved the Dockerfile.
2.  Build the Docker image with the `docker build` command. If your Dockerfile is named "Dockerfile" and
located in the current directory, you can use the command `docker build -t yourimagename .` where `yourimagename`
is the name you want to give to your Docker image. 
3.  After the Docker image has been built, you can run it using the `docker run` command. For instance,
you could use `docker run -d -p 8000:8000 yourimagename` to run the image in detached mode and map port 8000
in the container to port 8000 on your host machine. The `-d` flag tells Docker to run the container in the
background. The `-p` flag maps a network port in the container to a port on your host machine.
