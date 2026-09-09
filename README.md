# RTL-Terminal-RS v2.0 — نسخة Rust الاحترافية

> Terminal مكتوب بـ **Rust (Tauri v2)** مع دعم عربي/RTL كامل — بديل احترافي للنسخة Node/Electron السابقة.

## المعمارية
```
rtl-terminal-rs/
├── core/                       # Rust engine نقي (بدون webkit) — يُختبر هنا ✅
│   ├── bidi.rs                 # كشف RTL + إعادة ترتيب بصري
│   ├── shaping.rs              # تشكيل الحروف العربية + لام-ألف
│   └── pty.rs                  # PTY حقيقي عبر portable-pty
├── src-tauri/                  # تطبيق Tauri v2 (IPC ↔ frontend)
│   ├── main.rs                 # أوامر pty_spawn / write / resize / kill
│   ├── tauri.conf.json
│   ├── capabilities/
│   └── icons/
├── frontend/                   # WebView (xterm.js + RTL layer) — offline
├── .github/workflows/          # build تلقائي على Windows (GitHub Actions)
└── README.md
```

## ليه Rust/Tauri أرحب من Electron؟
| | Electron (v1.2) | Tauri v2 (هذا) |
|---|---|---|
| حجم التطبيق | ~143MB | ~10-15MB |
| الذاكرة | ~300MB | ~60MB |
| Shell | child_process (بدون PTY) | **PTY حقيقي** (portable-pty) |
| الأمان | node integration مخاطر | IPC commands محصورة |
| Frontend | Chromium مدمج | WebView نظامي (WebView2 / WebKitGTK) |

## البناء
```bash
cd src-tauri
cargo tauri build
```

## الاختبارات (على أي جهاز)
```bash
cd core
cargo test        # ✅ 7/7 passing
```

## GitHub Actions
بناء Windows exe تلقائي عند push أي `v*` tag:
`.github/workflows/build-windows.yml`

## الحالة
- ✅ `rtl-terminal-core` — compile + 7 unit tests pass (هنا)
- 🟡 `src-tauri` — كود كامل؛ يحتاج webkit2gtk-4.1-dev على Linux أو يبني على Windows/CI
- 🟡 installer NSIS + release تلقائي عبر tag

MIT — rust-memo.