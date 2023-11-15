FROM arm64v8/rust:1.66.1-slim-bullseye

ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]

RUN apt-get update -qq \
  && apt-get install -qy --no-install-recommends \
  clang \
  make \
  libcurl4 \
  openjdk-11-jdk-headless \
  ruby \
  ruby-dev \
  git \
  zip \
  curl \
  unzip \
  libtinfo6 \
  libncurses6 \
  python3-pip \
  && apt-get clean

RUN rustup self update
RUN rustup default nightly # for our project
RUN rustup toolchain install nightly-aarch64-unknown-linux-gnu
RUN rustup component add rustfmt
RUN rustup update
RUN rustup show

RUN mkdir -p /tmp/setup-swift \
  && cd /tmp/setup-swift \
  && curl -o swift.tar.gz https://download.swift.org/swift-5.7.3-release/ubuntu2004-aarch64/swift-5.7.3-RELEASE/swift-5.7.3-RELEASE-ubuntu20.04-aarch64.tar.gz \
  && tar -xzf swift.tar.gz \
  && mv swift-5.7.3-RELEASE-ubuntu20.04-aarch64 /opt/swift \
  && cd ../ \
  && rm -rf ./setup-swift

ENV PATH="$PATH:/opt/swift/usr/bin" 

RUN mkdir -p /tmp/setup-kotlin \
  && cd /tmp/setup-kotlin \
  && curl -o sdk.install.sh https://get.sdkman.io \
  && bash /tmp/setup-kotlin/sdk.install.sh \
  && . /root/.sdkman/bin/sdkman-init.sh \
  && sdk install kotlin \
  && cd ../ \
  && rm -rf ./setup-kotlin

RUN mkdir -p /tmp/setup-jna \
  && cd /tmp/setup-jna \
  && curl -o jna.jar https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar \
  && mv jna.jar /opt \
  && cd ../ \
  && rm -rf ./setup-jna

RUN gem install ffi --no-document
