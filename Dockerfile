# Build srvpru binary
FROM clux/muslrust as backend_builder
WORKDIR /usr/src/app/screen_switcher
COPY Cargo.* ./
COPY src/main.rs src/
RUN cargo fetch
COPY src/*.rs src/
RUN cargo build --release

FROM node as frontend_builder
WORKDIR /usr/src/app/screen_switcher
COPY package* ./
RUN npm install
COPY . .
RUN npm run build

FROM alpine
EXPOSE 8080
WORKDIR /usr/src/app/screen_switcher
ENV SCREEN_SWITCHER_BASE_URL="/usr/src/app/screen_switcher/dist"
ENV SCREEN_SWTICHER_TARGET="192.168.1.230:20107"
COPY --from=backend_builder /usr/src/app/screen_switcher/target/x86_64-unknown-linux-musl/release/screen_switcher .
COPY --from=frontend_builder /usr/src/app/screen_switcher/dist dist
ENTRYPOINT ["./screen_switcher"]
CMD ["serve"]
