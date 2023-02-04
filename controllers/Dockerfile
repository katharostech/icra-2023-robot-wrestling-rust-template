# Rust build stage
FROM cyberbotics/webots.cloud:R2023a-ubuntu20.04 as build

# Install Rust
RUN mkdir /tmp/installrust && \
    cd /tmp/installrust && \
    wget -q https://static.rust-lang.org/dist/rust-1.67.0-x86_64-unknown-linux-gnu.tar.gz && \
    tar -xf rust-* && \
    ./rust*/install.sh && \
    cd /tmp && \
    rm -rf installrust

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends libclang-dev

COPY participant /project
WORKDIR /project
RUN cargo b --release

# Runner stage
FROM cyberbotics/webots.cloud:R2023a-ubuntu20.04

# Environment variables needed for Webots
# https://cyberbotics.com/doc/guide/running-extern-robot-controllers#remote-extern-controllers
ENV LD_LIBRARY_PATH=${WEBOTS_HOME}/lib/controller:${LD_LIBRARY_PATH}
ARG WEBOTS_CONTROLLER_URL
ENV WEBOTS_CONTROLLER_URL=${WEBOTS_CONTROLLER_URL}

# Install the built participant from the previous build stage
RUN mkdir -p /usr/local/webots-project/controllers
COPY . /usr/local/webots-project/controllers
COPY --from=build /project/target/release/participant /usr/local/webots-project/controllers/participant/participant

# Entrypoint command to launch the controller
WORKDIR /usr/local/webots-project/controllers/participant
ENTRYPOINT ./participant
