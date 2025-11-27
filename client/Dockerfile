FROM node:20-alpine as builder

ARG VITE_SERVER_URL
ARG VITE_POLKADOT_NETWORK
ARG VITE_SIGNER_MNEMONIC

ENV VITE_SERVER_URL=$VITE_SERVER_URL
ENV VITE_POLKADOT_NETWORK=$VITE_POLKADOT_NETWORK
ENV VITE_SIGNER_MNEMONIC=$VITE_SIGNER_MNEMONIC

WORKDIR /app
COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
