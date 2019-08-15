FROM node:10.16.2-alpine

WORKDIR /src

RUN apk add tar
ADD package.json package.json
ADD package-lock.json package-lock.json
RUN npm install

ADD . /src

ENTRYPOINT ["npm", "start"]