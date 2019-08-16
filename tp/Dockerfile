FROM rustlang/rust:nightly-slim
WORKDIR /app

ENV PROTOBUF_VERSION=3.5.1 \
    RUST_PROTOBUF_VERSION=1.4.3

RUN apt-get update && apt-get install -y libssl-dev libzmq3-dev pkg-config curl autoconf automake libtool curl make g++ unzip

RUN mkdir -p /protobuf && \
    curl -L https://github.com/google/protobuf/archive/v${PROTOBUF_VERSION}.tar.gz | tar xvz --strip-components=1 -C /protobuf

RUN cd /protobuf && \
    autoreconf -f -i -Wall,no-obsolete && \
    ./configure --prefix=/usr --enable-static=no && \
    make -j2 && make install

COPY Cargo.toml .
COPY src/ src/

RUN USER=root cargo build 

CMD ["./target/debug/tp"]