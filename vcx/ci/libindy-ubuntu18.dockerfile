# Development
FROM ubuntu:18.04

# JRE installation and gcc
RUN apt-get update -y && apt-get install -y \
    gcc \
    pkg-config \
    build-essential \
    libsodium-dev \
    libssl-dev \
    libgmp3-dev \
    libsqlite3-dev \
    cmake \
    apt-transport-https \
    ca-certificates \
    software-properties-common \
    debhelper \
    wget \
    git \
    curl \
    libffi-dev \
    ruby \
    ruby-dev \
    rubygems \
    python3 \
    libtool \
    openjdk-8-jdk \
    maven \
    apt-transport-https \
    libzmq3-dev \
    zip \
    unzip \
    sudo

# Install Nodejs
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash - \
    && apt-get install -y nodejs

# Install Rust
ARG RUST_VER
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.40.0
ENV PATH /root/.cargo/bin:$PATH
RUN cargo install cargo-deb --color=never --version=1.21.1 #TEMPORARY - REMOVE WHEN 1.22 COMPILES

# Install Gradle
RUN wget -q https://services.gradle.org/distributions/gradle-3.4.1-bin.zip &&\
    mkdir /opt/gradle &&\
    unzip -q -d /opt/gradle gradle-3.4.1-bin.zip &&\
    rm gradle-3.4.1-bin.zip

# fpm for deb packaging of npm
RUN gem install fpm
RUN apt-get install rpm -y

COPY ./vcx/ci/scripts/installCert.sh /tmp
RUN /tmp/installCert.sh

# Add sovrin to sources.list
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88 && \
    add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable" && \
    add-apt-repository "deb https://repo.sovrin.org/sdk/deb bionic stable" && \
    add-apt-repository "deb https://repo.sovrin.org/sdk/deb bionic master"

# these are default values if they are not passed into the environment with
# the --build-arg flag from 'docker build' command.
ARG LIBINDY_VER=1.14.1
ARG LIBNULL_VER=1.14.1

RUN apt-get update && apt-get install -y \
    libindy=${LIBINDY_VER} \
    libnullpay=${LIBNULL_VER}