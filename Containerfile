FROM ubuntu:22.04

RUN apt update -y && \
 	apt install -y \
 	git \
 	lsb-release \
    curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable
RUN git clone https://github.com/BoosterRobotics/booster_robotics_sdk.git /booster_robotics_sdk/
WORKDIR /booster_robotics_sdk/
RUN git checkout 7fb7287070fd21e89c27422a8d8ab0a766e11853
RUN yes | ./install.sh
RUN mkdir build && cd build && cmake .. && make -j$(nproc)
WORKDIR /booster-dev/
ENV CARGO_TARGET_DIR /booster-dev-persistent/target/

