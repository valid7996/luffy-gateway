# Luffy Gateway — Field Testing Guide / راهنمای تست میدانی

## راهنمای تست میدانی روی اپراتورهای ایران (Persian)

به دلیل تفاوت‌های موجود در رفتار فیلترینگ عمیق (DPI)، اختلالات UDP و دستکاری DNS در اپراتورهای مختلف، این چک‌لیست برای راستی‌آزمایی کارکرد میدانی طراحی شده است:

### ۱. پیش‌نیازهای روی سرور
قبل از اتصال گوشی‌ها، دستور زیر را در سرور اجرا کنید تا از باز بودن پورت‌ها و درستی رکوردهای DNS مطمئن شوید:
```bash
luffy doctor your-tunnel-domain.com
luffy ports
```
همه موارد باید وضعیت `PASS` یا `LISTEN` داشته باشند.

### ۲. ماتریس تست اپراتورها

| اپراتور | نوع اتصال | موتور پیشنهادی | نتیجه تست | یادداشت‌ها |
| :--- | :--- | :--- | :--- | :--- |
| **همراه اول (MCI)** | 4G/5G | SlipGate (NoizDNS / DNSTT) با سایز ۸۰ یا ۶۰ | [ ] کار می‌کند | در صورت افت بسته، DoH/DoT فعال شود |
| **ایرانسل (MTN Irancell)** | 4G/5G | Soar یا MasterDnsVPN (حالت LeastLatency) | [ ] کار می‌کند | بررسی وضعیت پورت ۵۳ UDP |
| **رایتل (Rightel)** | 4G | SlipGate (VayDNS / Slipstream) | [ ] کار می‌کند | معمولاً کمترین اختلال روی رکوردهای سفارشی |
| **اینترنت ثابت (مخابرات / شاتل / آسیاتک)** | ADSL/VDSL/فیبر | GooseRelay (از طریق Google Relay) یا Soar | [ ] کار می‌کند | بهترین پایداری در اختلالات شدید سراسری |

### ۳. جمع‌آوری لاگ غیرحساس جهت گزارش
در صورت بروز اختلال در هر یک از اپراتورها، دستور زیر را جهت صدور فایل گزارش اجرا کنید (کلیدهای رمزنگاری و توکن‌ها خودکار پاکسازی می‌شوند):
```bash
luffy doctor your-tunnel-domain.com > field-report.txt
```

---

## Field Testing Guide (English)

Because real-world DPI behavior and packet-drop policies differ across carriers, use this verification matrix:

1. **Pre-flight on Server:** Run `luffy doctor <domain>` and ensure all 7 checks return `PASS`.
2. **Device Handshake:** Connect a real smartphone under each listed carrier and verify latency, packet-loss survival, and data transfer integrity.
3. **Report Generation:** Run `luffy doctor <domain> > field-report.txt` to export a sanitized diagnostic report.
