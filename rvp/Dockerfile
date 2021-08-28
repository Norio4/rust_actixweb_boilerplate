FROM nginx:1.17.8-alpine

ENV TZ=Asia/Tokyo

RUN apk update

COPY ./nginx.conf /etc/nginx/nginx.conf
COPY ./default.conf /etc/nginx/conf.d/default.conf
