# Development
FROM centos:7

ARG LIBINDY_VER
ARG LIBINDY_SIMPLE_VER
ARG LIBINDY_BRANCH

RUN yum install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm ;\
    yum install -y \
      python3 \
      git \
      zeromq \
      cargo \
      openssl-devel \
      rpm-build \
      https://repo.sovrin.org/rpm/libindy/${LIBINDY_BRANCH}/${LIBINDY_VER}/libindy.${LIBINDY_SIMPLE_VER}.rpm
