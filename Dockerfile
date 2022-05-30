FROM rust:1.61
RUN apt-get -y update
RUN apt-get -y install valgrind cmake
RUN rustup component add rustfmt

ENV APP_ROOT /usr/local/src/dlhn
RUN mkdir $APP_ROOT
WORKDIR $APP_ROOT
ADD . $APP_ROOT
RUN cargo test
