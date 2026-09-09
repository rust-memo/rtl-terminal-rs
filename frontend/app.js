// RTL-Terminal-RS frontend
// Transport: Tauri IPC (production) → WebSocket fallback (browser preview)
(function () {
  const termWrap = document.getElementById('term-wrap');
  const statusEl = document.getElementById('status');
  const statusT = document.getElementById('status-t');
  const modeBadge = document.getElementById('mode-badge');
  const rtlToggle = document.getElementById('rtl-toggle');
  const shapeToggle = document.getElementById('shape-toggle');
  const cmdInput = document.getElementById('cmd');

  let rtlEnabled = true;
  let shapeEnabled = true;
  let cols = 80, rows = 24;

  const term = new Terminal({
    cursorBlink: true,
    fontSize: 15,
    fontFamily: "'Cascadia Code','Segoe UI',Consolas,'Courier New',monospace",
    theme: {
      background: '#000000', foreground: '#e6edf3',
      cursor: '#3fb950', selectionBackground: '#264f78',
    },
    allowProposedApi: true,
  });
  const fit = new FitAddon.FitAddon();
  if (fit) term.loadAddon(fit);
  term.open(document.getElementById('term'));
  try { fit.fit(); } catch (e) {}
  window.addEventListener('resize', () => { try { fit.fit(); } catch (e) {} });
  cols = term.cols; rows = term.rows;

  term.writeln('\x1b[36mRTL-Terminal-RS — جارٍ الاتصال…\x1b[0m');

  const isTauri = typeof window.__TAURI__ !== 'undefined';
  let ws = null;
  let unlisten = null;

  function setStatus(on, txt) {
    statusEl.classList.toggle('on', !!on);
    statusT.textContent = txt;
  }
  function writeOut(data) {
    let out = data;
    if (rtlEnabled) out = RTLBidi.processOutput(out);
    term.write(out);
  }

  // ---------- Tauri IPC path (poll-based) ----------
  async function startTauri() {
    const { invoke } = window.__TAURI__.core;
    await invoke('pty_start');
    setInterval(async () => {
      try {
        const s = await invoke('pty_poll');
        if (s && s.length) writeOut(s);
      } catch (e) {}
    }, 30);
    term.onData((d) => { try { invoke('pty_write', { data: d }); } catch (e) {} });
    term.onResize(({ cols: c, rows: r }) => { try { invoke('pty_resize', { cols: c, rows: r }); } catch (e) {} });
    setStatus(true, 'متصل ✓');
    modeBadge.textContent = 'Tauri IPC';
    term.writeln('\x1b[32m✓ PTY جاهز — Rust backend\x1b[0m');
  }

  // ---------- WebSocket fallback (browser demo) ----------
  function startWs() {
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    ws = new WebSocket(`${proto}://${location.host}`);
    ws.onopen = () => {
      setStatus(true, 'متصل ✓');
      modeBadge.textContent = 'WebSocket (demo)';
      term.writeln('\x1b[32m✓ متصل (وضع المتصفح)\x1b[0m');
      ws.send(JSON.stringify({ type: 'resize', cols: term.cols, rows: term.rows }));
    };
    ws.onclose = () => {
      setStatus(false, 'غير متصل');
      term.writeln('\r\n\x1b[31m✗ انقطع الاتصال.\x1b[0m');
    };
    ws.onmessage = (ev) => {
      try {
        const m = JSON.parse(ev.data);
        if (m.type === 'data') writeOut(m.data);
      } catch (e) {}
    };
    term.onData((d) => {
      if (ws && ws.readyState === 1) ws.send(JSON.stringify({ type: 'input', data: d }));
    });
    term.onResize(({ cols: c, rows: r }) => {
      if (ws && ws.readyState === 1) ws.send(JSON.stringify({ type: 'resize', cols: c, rows: r }));
    });
  }

  // start transport
  if (isTauri) {
    startTauri().catch((e) => {
      modeBadge.textContent = 'fallback';
      startWs();
    });
  } else {
    startWs();
  }

  // ---------- smart input box ----------
  function sendCmd() {
    const v = cmdInput.value;
    if (!v) return;
    if (isTauri) {
      window.__TAURI__.core.invoke('pty_write', { data: v + '\r' });
    } else if (ws && ws.readyState === 1) {
      ws.send(JSON.stringify({ type: 'input', data: v + '\r' }));
    }
    cmdInput.value = '';
    cmdInput.focus();
  }
  document.getElementById('send-btn').onclick = sendCmd;
  cmdInput.addEventListener('keydown', (e) => { if (e.key === 'Enter') sendCmd(); });
  cmdInput.addEventListener('input', () => {
    cmdInput.setAttribute('dir', RTLBidi.isRTL(cmdInput.value) ? 'rtl' : 'ltr');
  });

  // ---------- toggles ----------
  rtlToggle.onchange = () => {
    rtlEnabled = rtlToggle.checked;
    termWrap.classList.toggle('rtl-mode', rtlEnabled);
    document.documentElement.setAttribute('dir', rtlEnabled ? 'rtl' : 'ltr');
  };
  shapeToggle.onchange = () => { shapeEnabled = shapeToggle.checked; };
  document.getElementById('clear-btn').onclick = () => term.clear();
  document.getElementById('dir-btn').onclick = () => {
    const cur = document.documentElement.getAttribute('dir');
    const next = cur === 'rtl' ? 'ltr' : 'rtl';
    document.documentElement.setAttribute('dir', next);
    rtlToggle.checked = next === 'rtl';
    rtlEnabled = rtlToggle.checked;
    termWrap.classList.toggle('rtl-mode', rtlEnabled);
  };

  termWrap.classList.add('rtl-mode');
  term.onKey(({ domEvent }) => {
    if (domEvent.ctrlKey && domEvent.key === 'l') { term.clear(); domEvent.preventDefault(); }
  });
})();