FROM wasm_builder:v0.1.0 as builder
WORKDIR /root/share/repository/moon_web
COPY . .
RUN /root/.cargo/bin/trunk build --release

FROM light:v0.1.12-p
COPY --from=builder /root/share/repository/moon_web/dist/ /root/share/server/moon_web/dist
WORKDIR /root/share/server/moon_web
EXPOSE 80
CMD ["light"]
