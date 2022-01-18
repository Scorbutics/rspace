FROM rust:latest

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends libsdl2-dev libsdl2-mixer-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-gfx-dev build-essential
WORKDIR /root/dev/rspace
COPY . .

RUN cargo build --release

FROM debian:latest  
WORKDIR /root/rspace
COPY --from=0 /root/dev/rspace/target/release/* ./
COPY --from=0 /root/dev/rspace/*.png ./
COPY --from=0 /root/dev/rspace/*.ttf ./

RUN apt-get update && apt-get install -y --no-install-recommends libsdl2-dev libsdl2-mixer-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-gfx-dev

CMD ./playground