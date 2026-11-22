# 本地 TLS 证书

项目默认不启用 TLS。需要本地 HTTPS 调试时，在项目根目录执行：

```sh
openssl req -x509 -newkey rsa:2048 -nodes -days 3650 \
  -subj "/CN=asdfri.local" \
  -addext "subjectAltName=DNS:asdfri.local,DNS:localhost,IP:127.0.0.1,IP:::1" \
  -addext "basicConstraints=critical,CA:FALSE" \
  -addext "keyUsage=critical,digitalSignature,keyEncipherment" \
  -addext "extendedKeyUsage=serverAuth" \
  -keyout self_signed_certs/asdfri.key \
  -out self_signed_certs/asdfri.crt
```

生成的私钥已被 `.gitignore` 忽略，请勿提交到版本库。将 `config.toml` 中
`[server.tls].enabled` 设为 `true` 后重启服务。
