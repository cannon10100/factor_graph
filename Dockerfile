FROM ubuntu:16.04

# Update
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y curl gcc

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Put rust executables in path
RUN echo PATH="${PATH}:~/.cargo/bin/" >> ~/.bashrc

# Copy in code
RUN mkdir /factor_graph
COPY . /factor_graph

# Build the project 
RUN ~/.cargo/bin/cargo run --manifest-path /factor_graph/Cargo.toml
CMD cp /factor_graph/factor_graph.dot /graphs
