# Development
FROM centos:7

ARG LIBINDY_VER
ARG LIBINDY_SIMPLE_VER
ARG LIBINDY_BRANCH
ARG RUST_VER

RUN yum install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm ;\
    yum install -y \
      python3 \
      git \
      zeromq \
      gcc \
      openssl-devel \
      rpm-build \
      https://repo.sovrin.org/rpm/libindy/${LIBINDY_BRANCH}/${LIBINDY_VER}/libindy.${LIBINDY_SIMPLE_VER}.rpm

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VER}
ENV PATH /root/.cargo/bin:$PATH
