FROM node:22.13-bookworm

WORKDIR /app
COPY ./application /app

RUN  npm install
# RUN npx next build

# EXPOSE 3000

# CMD [ "npx", "next", "start" ]
