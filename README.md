# Hermes Code Execution Engine

Hermes is a high-performance code execution engine written in Rust. It provides sandboxed, on-demand execution of arbitrary code through a simple REST API.

The system is designed for judge platforms, coding sandboxes and auto-eval services. 
It runs untrusted and potentially hostile code inside isolated environments with strict resource limits to ensure safety, predictability, and throughput.

## Setup Instruction (Local)

### Requirements
* Rust toolchain installed

### Build application
```bash
git clone https://github.com/Ryuga/Hermes.git
cd Hermes
cargo build
```
### create `.env` file with following config
```shell
DEBUG=true  # turns on log output through std_log
HOST=127.0.0.1
PORT=8000
ALLOWED_ORIGIN=https://your_frontend_domain_to_allow_cors.com
```
### Run the application
```shell
cargo run
```
Application will be live at `http://127.0.0.1:8000`

Validate `curl http://127.0.0.1` should return `UP!`

---
## Production Deployment Instructions 

### Requirements
* Linux server
* Rust toolchain installed
* Nginx installed

### Build Application

```bash
git clone https://github.com/Ryuga/Hermes.git
cd Hermes
cargo build --release
```
Binary:
```
target/release/Hermes
```

### create `.env` file with following config
```shell
DEBUG=false  # turns off log output through std_log
HOST=127.0.0.1
PORT=8000
ALLOWED_ORIGIN=https://your_frontend_domain_for_cors.com
```

### Configure Axum Bind Address

Bind to localhost or all interfaces:

```rust
TcpListener::bind("127.0.0.1:8000")
```

### Test App

```bash
./target/release/Hermes
curl http://127.0.0.1:8000
```

### Install Nginx

```bash
sudo apt update
sudo apt install nginx
sudo systemctl enable nginx
sudo systemctl start nginx
```

### Install Certificate on Server (Optional)

```bash
sudo mkdir -p /etc/nginx/certs
sudo nano /etc/nginx/certs/origin.crt
sudo nano /etc/nginx/certs/origin.key
```

Paste contents and secure key:
```bash
sudo chmod 600 /etc/nginx/certs/origin.key
```

### Nginx HTTPS Reverse Proxy Config

```bash
sudo nano /etc/nginx/sites-available/hermes
```

```
server {
    listen 443 ssl; 
    # listen 80; 
    server_name api.hermes.domain;

    ssl_certificate /etc/nginx/certs/origin.crt;
    ssl_certificate_key /etc/nginx/certs/origin.key;

    location / {
        proxy_pass http://127.0.0.1:8000;

        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
    }
}
```

Enable site:

```bash
sudo ln -s /etc/nginx/sites-available/hermes /etc/nginx/sites-enabled/
```

Test:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

---

## Run App as Service

```bash
sudo nano /etc/systemd/system/hermes.service
```

```
[Unit]
Description=Hermes Engine
After=network.target

[Service]
User=www-data
WorkingDirectory=/path/to/your/project/root
ExecStart=/path/to/your/project/root/target/release/Hermes
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

Enable:

```bash
sudo systemctl daemon-reload
sudo systemctl enable hermes
sudo systemctl start hermes
```

### Logs

```
journalctl -u hermes -f
/var/log/nginx/error.log
```

## Code Execution API

Execute user code in a sandboxed environment. Currently supports:

* Python
* JavaScript

---

### Endpoint

```
POST /execute
```

---

### Request

**Headers**

```
Content-Type: application/json
```

**Body**

```json
{
  "language": "python | javascript",
  "code": "your source code"
}
```

---

### Response

```json
{
  "code": 0,
  "output": "stdout text",
  "std_log": "stderr text/ std_log text"
}
```

**Fields**

* `code` → exit code (`0` = success, non-zero = runtime error)
* `output` → program stdout
* `std_log` → error output / logs if `DEBUG=true`

---

### Example

```bash
curl -X POST https://your-api/execute \
  -H "Content-Type: application/json" \
  -d '{"language":"python","code":"print(1+1)"}'
```

---

### API Limits
* Default Nginx rate limiting
---

### Currently used by
* Tortoise Community Online Code Execution Tool
* Tortoise-BOT for discord code execution utility.

