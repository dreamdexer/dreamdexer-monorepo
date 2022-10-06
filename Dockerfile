

# ## Builder
FROM rust:latest as builder

ARG SERVICE
ENV SERVICE ${SERVICE}

WORKDIR /app

COPY . .

RUN cargo build -p $SERVICE --release

## Final image
FROM debian:buster-slim
RUN apt-get update && apt-get install libssl1.1
ARG SERVICE
ENV SERVICE ${SERVICE}
COPY --from=builder ./app/target/release/$SERVICE ./usr/local/bin/$SERVICE
WORKDIR /usr/local/bin/
CMD ${SERVICE}