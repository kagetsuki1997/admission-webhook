cargo run --bin admission-webhook run \
--doge-default-image doge/doge:doge \
--doge-default-number 87 \
--doge-default-status Normal \
--tls-cert examples/self_signed_certs/cert.pem \
--tls-key examples/self_signed_certs/key.pem