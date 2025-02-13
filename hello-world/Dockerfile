FROM postgres:17.2

RUN apt-get update && apt-get install -y \
    libicu-dev \
    && rm -rf /var/lib/apt/lists/*

ENV PG_ICU_LOCALE en-US

CMD ["postgres"]