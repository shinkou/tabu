FROM rust AS build
ADD ./ /data
WORKDIR /data
RUN cargo build

FROM debian
COPY --from=build /data/target/debug/tabu /usr/local/bin/tabu
ENTRYPOINT ["/usr/local/bin/tabu"]
