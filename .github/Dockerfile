FROM ubuntu:latest

RUN apt-get update && apt-get -y install curl
RUN curl https://sh.rustup.rs -sSf | bash -s -- '-y'
RUN apt-get -y install build-essential pkg-config cmake libasound2-dev python3 libfreetype6-dev libexpat1-dev libx11-dev libx11-xcb-dev libxcb-xfixes0-dev libxcb-shape0-dev libxcb-render0-dev