####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

# Create unprivileged user
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

RUN make build-release

####################################################################################################
## Final image
####################################################################################################

# Version pinned so that it can be maintained by renovate bot.
FROM debian:12.11-slim

# Import user from builder.
COPY --from=builder --chmod=444 /etc/passwd /etc/passwd
COPY --from=builder --chmod=444 /etc/group /etc/group

WORKDIR /potree-auth

# Copy our build
COPY --from=builder /potree-auth/target/release/potree-auth ./

# Use an unprivileged user.
USER potree-auth:potree-auth

# All arguments to come from environment variables.
CMD ["/potree-auth/potree-auth"]