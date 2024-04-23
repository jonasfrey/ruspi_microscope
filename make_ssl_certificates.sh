# Generate a private key
openssl genrsa -out key.pem 2048

# Generate a certificate signing request
openssl req -new -key key.pem -out request.csr -subj "/CN=127.0.0.1"

# Generate a self-signed certificate from the request
openssl x509 -req -days 365 -in request.csr -signkey key.pem -out cert.pem
