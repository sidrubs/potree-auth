####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

# Create appuser
ENV USER=potree-auth
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /potree-auth

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder --chmod=444 /etc/passwd /etc/passwd
COPY --from=builder --chmod=444 /etc/group /etc/group

WORKDIR /potree-auth

# Copy our build
COPY --from=builder /potree-auth/target/x86_64-unknown-linux-musl/release/potree-auth ./

# Use an unprivileged user.
USER potree-auth:potree-auth

CMD ["/potree-auth/potree-auth"]