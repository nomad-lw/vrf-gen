FROM rust:1.86 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM almalinux:9 as runner
RUN dnf install -y openssl-devel && dnf clean all
COPY --from=builder /usr/src/app/target/release/vrf_listener /usr/local/bin/
CMD ["vrf_listener"]
