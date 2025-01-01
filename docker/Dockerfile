# Use the specified Rust version and base OS as the base image
ARG BASE_OS
ARG RUST_VERSION
FROM rust:${RUST_VERSION}-${BASE_OS}

# Nodejs installation
RUN ["mkdir", "/usr/local/nvm"]
ENV NVM_DIR /usr/local/nvm

RUN ["apt", "update"]
RUN ["apt", "install", "curl", "-y", "--no-install-recommends"]

RUN ["curl", "https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh", "-o", "/tmp/install_nvm.sh"]
RUN ["bash", "/tmp/install_nvm.sh"]

ARG NODE_VERSION
RUN ["sh", "-c", ". /usr/local/nvm/nvm.sh && nvm install ${NODE_VERSION}"]

# Ensure Node.js binaries are available in the PATH
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# Install pnpm and turbo globally
RUN ["npm", "install", "-g", "pnpm"]
RUN ["mkdir", "/pnpm"]
ENV PATH=/pnpm:$PATH
ENV PNPM_HOME=/pnpm
RUN ["pnpm", "install", "-g", "turbo"]

# Install cargo-watch for Rust live-reloading
RUN ["cargo", "install", "cargo-watch"]

# Set the working directory to /app
WORKDIR /app

# Copy project files into the container (excepted files/dir declared in .dockerignore)
COPY . .

# Install Node.js dependencies (defined in package.json)
RUN ["pnpm", "install"]

# Compile and add the directory with compiled binaries to the PATH for direct access
RUN ["cargo", "build"]
ENV PATH=/app/target/debug:$PATH