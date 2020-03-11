FROM rust:1.41

USER dev
ENV HOME /home/dev
ENV USER dev
ENV SHELL /bin/bash
WORKDIR /home/dev

RUN curl -sSf sh.rustup.rs | sh -s -- -y
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
RUN echo "export PS1='\u:\w$ '" >> ~/.bashrc