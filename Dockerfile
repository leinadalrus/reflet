FROM rust:1.52.1
# might be better to use rust:1.53
WORKDIR src
RUN cargo build --release
ENTRYPOINT [ "./target/release/reflet" ]
# docker build --tag reflet --file Dockerfile .