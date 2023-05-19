FROM ruby:3.2.2-alpine

RUN apk add build-base

RUN adduser -D gemserver
USER gemserver
WORKDIR /home/gemserver

RUN gem install geminabox puma
RUN mkdir data

COPY ./config.ru .

EXPOSE 9292

CMD ["rackup"]

