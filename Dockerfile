FROM centos:8 as BUILD

RUN yum install -y curl gcc gcc-c++ make
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:$PATH"

ADD . ./

RUN cargo build --release

FROM centos:8

COPY --from=BUILD /target/release/linker /usr/local/bin/

ENV ROCKET_ENV=production
ENV ROCKET_PORT=$PORT

WORKDIR /root
CMD ["/usr/local/bin/linker"]
ENTRYPOINT ["sh", "-c"]