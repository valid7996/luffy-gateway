# Installation Guide · راهنمای نصب Luffy Gateway

> **فارسی** پایین این صفحه · **English** starts right here
>
> Target platform: **Ubuntu 22.04/24.04 · Debian 12/13** (x86_64 / aarch64)
> Windows/macOS are supported for **development and testing only**.

---

## English

### 1. Requirements

| Requirement | Notes |
|---|---|
| Linux server (Ubuntu 22.04+/Debian 12+) | A VPS with **at least 1 GB RAM** and a public IPv4 |
| `curl`, `git`, `build-essential` | Base toolchain |
| Rust **stable 1.85+** | `curl https://sh.rustup.rs -sSf \| sh` |
| `root` (sudo) | Needed for port 53 and systemd units |

### 2. Build from source

```bash
git clone https://github.com/LuffyGateway/luffy-gateway.git
cd luffy-gateway
cargo build --release
```

The single binary lands at `target/release/luffy`.

### 3. Install to system paths

```bash
sudo install -m 755 target/release/luffy /usr/local/bin/luffy
sudo useradd --system --no-create-home --shell /usr/sbin/nologin luffy 2>/dev/null || true
sudo mkdir -p /etc/luffy /var/lib/luffy /var/log/luffy
sudo chmod 700 /etc/luffy /var/lib/luffy
```

### 4. Systemd services

The CLI can write hardened unit files itself (`CAP_NET_BIND_SERVICE`,
`NoNewPrivileges=true`, `ProtectSystem=strict`):

```bash
sudo luffy install
```

Or create them manually:

**`/etc/systemd/system/luffy-router.service`** — DNS front router (owns UDP/TCP :53):

```ini
[Unit]
Description=Luffy Gateway DNS Front Router
After=network.target

[Service]
Type=simple
User=luffy
Group=luffy
ExecStart=/usr/local/bin/luffy router --bind 0.0.0.0:53 --route example.com=127.0.0.1:5310 --default 8.8.8.8:53
Restart=always
RestartSec=3s
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

**`/etc/systemd/system/luffy-panel.service`** — web panel (loopback + reverse proxy):

```ini
[Unit]
Description=Luffy Gateway Control Panel
After=network.target luffy-router.service

[Service]
Type=simple
User=luffy
Group=luffy
ExecStart=/usr/local/bin/luffy serve 8080 --password CHANGE_ME
Restart=always
RestartSec=5s
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

Enable both:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now luffy-router.service luffy-panel.service
```

### 5. Firewall (UFW)

```bash
sudo ufw allow 53/udp comment 'luffy-managed'
sudo ufw allow 53/tcp comment 'luffy-managed'
sudo ufw allow 8080/tcp comment 'luffy-managed'   # panel (recommend TLS reverse proxy instead)
```

### 6. Verify the installation

```bash
luffy doctor your-tunnel-domain.com YOUR_SERVER_IP   # 7-point diagnostics
luffy ports                                          # probe managed ports
curl "http://127.0.0.1:8080/sub/<token>"             # your subscription link
```

The panel prints the **demo subscription URL** once at startup. Save it —
only its SHA-256 hash is stored internally.

### 7. Security checklist before exposing the panel

- [ ] Panel runs on `127.0.0.1` (default) behind **nginx/caddy with TLS**, or explicitly
      started with `--host 0.0.0.0 --password <strong-password>`
- [ ] `--password` set for admin pages (HTTP Basic auth)
- [ ] Tunnel domains have **grey-cloud** (DNS-only) records — Cloudflare proxy
      breaks UDP/53 (run `luffy doctor` to confirm)
- [ ] SSH key-only login enabled

### 8. Troubleshooting

| Symptom | Fix |
|---|---|
| `Failed to bind 0.0.0.0:53` | `sudo systemctl disable --now systemd-resolved` then `sudo systemctl start luffy-router` |
| Doctor reports orange-cloud Fail | Set the tunnel record to *DNS only* in your DNS provider |
| Panel unreachable remotely | Panel binds loopback by default; use a reverse proxy or pass `--host 0.0.0.0 --password …` |
| Subscription 404 | Token is a capability — copy the exact URL printed at startup |

---

## فارسی

### ۱. پیش‌نیازها

| نیازمندی | توضیح |
|---|---|
| سرور لینوکسی (Ubuntu 22.04+ / Debian 12+) | یک VPS با حداقل **۱ گیگ رم** و IPv4 عمومی |
| `curl`، `git`، `build-essential` | ابزار پایه |
| Rust **Stable 1.85 به بالا** | `curl https://sh.rustup.rs -sSf \| sh` |
| دسترسی `root` (sudo) | برای پورت ۵۳ و یونیت‌های systemd |

### ۲. ساخت از سورس

```bash
git clone https://github.com/LuffyGateway/luffy-gateway.git
cd luffy-gateway
cargo build --release
```

باینری نهایی در مسیر `target/release/luffy` قرار می‌گیرد.

### ۳. نصب در مسیرهای سیستمی

```bash
sudo install -m 755 target/release/luffy /usr/local/bin/luffy
sudo useradd --system --no-create-home --shell /usr/sbin/nologin luffy 2>/dev/null || true
sudo mkdir -p /etc/luffy /var/lib/luffy /var/log/luffy
sudo chmod 700 /etc/luffy /var/lib/luffy
```

### ۴. سرویس‌های Systemd

نصب خودکار با یونیت‌های سخت‌شده (CAP_NET_BIND_SERVICE، NoNewPrivileges، ProtectSystem=strict):

```bash
sudo luffy install
```

یا به‌صورت دستی دو فایل یونیت بسازید:

**`/etc/systemd/system/luffy-router.service`** — روتر فرانت DNS (مالک UDP/TCP پورت ۵۳):

```ini
[Unit]
Description=Luffy Gateway DNS Front Router
After=network.target

[Service]
Type=simple
User=luffy
Group=luffy
ExecStart=/usr/local/bin/luffy router --bind 0.0.0.0:53 --route example.com=127.0.0.1:5310 --default 8.8.8.8:53
Restart=always
RestartSec=3s
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

**`/etc/systemd/system/luffy-panel.service`** — پنل وب (لوپ‌بک + پروکسی معکوس):

```ini
[Unit]
Description=Luffy Gateway Control Panel
After=network.target luffy-router.service

[Service]
Type=simple
User=luffy
Group=luffy
ExecStart=/usr/local/bin/luffy serve 8080 --password CHANGE_ME
Restart=always
RestartSec=5s
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

فعال‌سازی هر دو سرویس:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now luffy-router.service luffy-panel.service
```

### ۵. فایروال (UFW)

```bash
sudo ufw allow 53/udp comment 'luffy-managed'
sudo ufw allow 53/tcp comment 'luffy-managed'
sudo ufw allow 8080/tcp comment 'luffy-managed'   # برای پنل (پیشنهاد: پروکسی معکوس TLS)
```

### ۶. تأیید نصب

```bash
luffy doctor your-tunnel-domain.com YOUR_SERVER_IP   # تشخیص ۷ نقطه‌ای
luffy ports                                          # بررسی پورت‌های مدیریت‌شده
curl "http://127.0.0.1:8080/sub/<token>"             # لینک اشتراک
```

پنل در لحظه استارت، **URL اشتراک دمو** را یک بار چاپ می‌کند. آن را ذخیره کنید —
به‌صورت داخلی فقط هش SHA-256 توکن نگه‌داری می‌شود.

### ۷. چک‌لیست امنیتی قبل از عمومی‌کردن پنل

- [ ] پنل روی `127.0.0.1` (پیش‌فرض) پشت **nginx/caddy با TLS** یا به‌طور آگاهانه با
      `--host 0.0.0.0 --password <رمز قوی>` اجرا شده باشد
- [ ] `--password` برای صفحات مدیریتی تنظیم شده باشد (احراز هویت Basic)
- [ ] دامنه‌های تونل **ابر خاکستری** (DNS-only) باشند — پراکسی Cloudflare ترافیک UDP/53
      را می‌شکند (`luffy doctor` را اجرا کنید)
- [ ] ورود SSH فقط با کلید فعال باشد

### ۸. رفع اشکال

| نشانه | راه‌حل |
|---|---|
| خطای `Failed to bind 0.0.0.0:53` | `sudo systemctl disable --now systemd-resolved` سپس `sudo systemctl start luffy-router` |
| Doctor گزارش Fail برای ابر نارنجی | رکورد دامنه تونل را روی *DNS only* تنظیم کنید |
| پنل از راه دور باز نمی‌شود | پنل پیش‌فرض روی لوپ‌بک bind می‌شود؛ از پروکسی معکوس استفاده کنید یا `--host 0.0.0.0 --password …` بدهید |
| اشتراک 404 می‌دهد | توکن یک قابلیت یکتاست؛ URL دقیق چاپ‌شده در استارت را کپی کنید |

---

## Next steps · گام‌های بعدی

- [User Guide (English)](docs/USER_GUIDE_EN.md)
- [User Guide (فارسی)](docs/USER_GUIDE_FA.md)
- [Field Testing for Iranian Carriers](FIELD-TEST.md)
- [Architecture](docs/ARCHITECTURE.md) · [Verification Matrix](VERIFICATION.md)
