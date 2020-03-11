FROM rust:1.41

RUN groupadd dev -g 999
RUN useradd dev -g dev -d /home/dev -m

WORKDIR /var/www/dreamsbird_api

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
RUN echo "export PS1='\u:\w$ '" >> ~/.bashrc