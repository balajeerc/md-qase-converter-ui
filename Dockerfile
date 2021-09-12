FROM debian:latest

RUN apt-get update
RUN apt-get install -y curl make build-essential
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y
ENV PATH=/root/.cargo/bin:$PATH

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

RUN mkdir -p /usr/local/nvm
ENV NVM_DIR /usr/local/nvm
ENV NODE_VERSION 14.17.6
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash

RUN rm /bin/sh && ln -s /bin/bash /bin/sh
RUN source $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

ENV NODE_PATH=$NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH=$NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

WORKDIR /usr/src/app

COPY rust rust
RUN mkdir -p /usr/src/app/rust/pkg
RUN (cd /usr/src/app/rust && wasm-pack build)

COPY package.json package.json
COPY package-lock.json package-lock.json
RUN npm install

COPY tsconfig.json tsconfig.json
COPY next-env.d.ts next-env.d.ts
COPY public public
COPY styles styles

COPY next.config.js next.config.js

COPY components components
COPY pages pages

RUN npm run build

CMD ["npm", "run", "start"]

