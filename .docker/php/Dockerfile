FROM php:7.4-fpm

RUN apt-get update && apt-get install -y \
      wget \
      git \
      fish

RUN apt-get update && apt-get install -y libpq-dev libzip-dev libicu-dev && docker-php-ext-install zip intl

COPY php.ini /etc/php/7.4/php.ini
COPY php-fpm-pool.conf /etc/php/7.4/pool.d/www.conf

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -y | sh && source $HOME/.cargo/env

RUN groupadd dev -g 999
RUN useradd dev -g dev -d /home/dev -m

WORKDIR /var/www/dreamsbird_api

EXPOSE 9000