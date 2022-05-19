# syntax=docker/dockerfile:1.4
###################################################################################################
## Builder
####################################################################################################
FROM rust:slim-bullseye AS builder

COPY scripts /tmp/scripts
RUN /tmp/scripts/install-deps

WORKDIR /build
COPY . /build

ARG DEBUG=0
ARG CARGO_ARGS=""
ARG BIN_NAMES="admission-webhook"

ENV RUST_BACKTRACE=1

COPY vendor/ .

RUN <<EOF
#!/usr/bin/env bash
set -eu

if [ "${DEBUG}" = "1" ]; then
  cargo build ${CARGO_ARGS}
else
  cargo build ${CARGO_ARGS} --release
fi

for BIN_NAME in $BIN_NAMES; do
  echo "${BIN_NAME}:"

  if [ "${DEBUG}" = "1" ]; then
    cp -v target/debug/${BIN_NAME} /usr/bin
  else
    cp target/release/${BIN_NAME} /usr/bin
    INITIAL_SIZE="$(sh -c 'sz="$(du -sk /usr/bin/'${BIN_NAME}')" ; echo "${sz%%[[:space:]]*}"')"
    strip /usr/bin/${BIN_NAME}
    FINAL_SIZE="$(sh -c 'sz="$(du -sk /usr/bin/'${BIN_NAME}')" ; echo "${sz%%[[:space:]]*}"')"
    REMOVED_SIZE=$((INITIAL_SIZE - FINAL_SIZE))
    echo "Cleaning process removed ${REMOVED_SIZE}KB"
    echo "Dropped binary size from ${INITIAL_SIZE}KB to ${FINAL_SIZE}KB"
  fi

  file /usr/bin/${BIN_NAME}
  ldd /usr/bin/${BIN_NAME}
  /usr/bin/${BIN_NAME} --version

done

EOF

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

USER 8787:8787

ENV RUST_BACKTRACE=${DEBUG}

# Import from builder.
COPY --from=builder /usr/bin/admission-webhook /usr/bin

ENTRYPOINT [ "/usr/bin/admission-webhook" ]
