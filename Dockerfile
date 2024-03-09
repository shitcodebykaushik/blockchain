FROM rust:1.76
WORKDIR /usr/src/blockchain
 COPY . . 
 RUN cargo install --path .
 
CMD [ "My blockchain" ]