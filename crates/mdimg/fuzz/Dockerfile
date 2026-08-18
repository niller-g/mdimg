FROM rustlang/rust:nightly

RUN cargo install cargo-fuzz

COPY . /app
WORKDIR /app/crates/mdimg
RUN cargo fuzz build

ENTRYPOINT ["cargo", "fuzz", "run"]
CMD ["replace_images", "--", "-max_total_time=60"]
