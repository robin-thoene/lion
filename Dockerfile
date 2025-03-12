FROM node:23 as node-builder
WORKDIR /app
COPY . .
RUN npm i
RUN npx @tailwindcss/cli -i ./styles/main.css -o ./static/css/main.css --minify

FROM rust:1.85 as builder
WORKDIR /app
COPY . .
COPY --from=node-builder ./app/node_modules ./node_modules
COPY --from=node-builder ./app/static/css/main.css ./static/css/main.css
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release .
COPY --from=builder /app/static ./static
COPY --from=builder /app/locales ./locales
COPY --from=builder /app/node_modules ./node_modules
ENV BIND_ADDR="0.0.0.0:3000"
EXPOSE 3000
CMD ["./lion"]
