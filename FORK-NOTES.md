# Fork 說明 / Fork Notice

本 repo 是 [herdrdev/herdr](https://github.com/herdrdev/herdr) 的 fork，授權沿用 Apache
License 2.0（見 `LICENSE`）。依 Apache-2.0 第 4(b) 條，以下列出本 fork 相對上游的修改。

基準版本：**v0.9.0**（上游 commit `b99002ac`）

This is a fork of [herdrdev/herdr](https://github.com/herdrdev/herdr), licensed under the
Apache License 2.0 (see `LICENSE`). Per Apache-2.0 §4(b), the modifications relative to
upstream are listed below. Base: **v0.9.0** (upstream commit `b99002ac`).

---

## 1. 繁體中文（台灣）介面 / Traditional Chinese UI

格式為 `English(繁體中文)`：半形括號、括號前後無空格、中文置於英文之後。原英文字串
一律保留，讓與上游的 diff 仍可讀、升級時不脫節。

- `src/ui/bilingual.rs`（新增）：靜態查表 `bilingual(english) -> &str`，供 keybind help
  與 settings 選項使用。
- 其餘面板、提示、通知等逐檔改寫字面字串。
- 受影響的測試斷言改用 `starts_with` / `contains`，不比對整串。

Format is `English(繁體中文)` — half-width parentheses, no surrounding space, Chinese after
the English. The original English is always kept so the diff against upstream stays readable.

## 2. Windows 關閉行為修正 / Windows shutdown fixes

**問題**：關閉 pane 時，正在該 pane 執行的安裝程式（winget、msiexec、*setup.exe）會被
中斷，造成安裝失敗或檔案殘缺。

- **受保護程序不送訊號**：pane 關閉時，若子程序屬安裝程式類，跳過終止。名單為
  `msiexec.exe`、`winget.exe`、`wusa.exe`、`dism.exe`、`trustedinstaller.exe`、
  `tiworker.exe`、`pkgmgr.exe`，以及以 `setup.exe` / `install.exe` / `installer.exe` /
  `uninstall.exe` / `uninstaller.exe` 結尾者、或 Inno Setup 的 `unins###.exe`。
- **PTY 延後關閉**：關閉 ConPTY handle 會讓 conhost 對 console 程式送 `CTRL_CLOSE`
  （結束碼 `0xC000013A`），這才是安裝程式被中斷的真正機制，單純不呼叫
  `TerminateProcess` 沒有用。改為偵測到受保護的後代程序時延後關閉 PTY，每秒重掃一次
  程序清單，上限 30 分鐘（可用環境變數 `HERDR_SHUTDOWN_HOLD_MAX_SECS` 覆蓋）。
- **修正上游潛在 bug**：`TerminateProcess` 原本是對以
  `PROCESS_QUERY_LIMITED_INFORMATION` 開啟的 handle 呼叫（權限不足，必然失敗），且未
  檢查回傳值。改為以 `PROCESS_TERMINATE` 開啟並檢查結果。
- Server 結束前呼叫 `wait_for_deferred_pane_shutdowns()`，確保延後關閉的 pane 收尾完成
  才真正離開。

Closing a pane used to interrupt installers running in it. Two causes were fixed: the
signal ladder now skips installer-class processes, and the ConPTY handle close itself is
deferred while protected descendants are alive (re-scanned every second, capped at 30
minutes) — closing it sends `CTRL_CLOSE` to console programs, which was the actual
mechanism. Also fixed an upstream latent bug where `TerminateProcess` was called on a
handle opened without `PROCESS_TERMINATE`, with its return value unchecked.

## 3. 新增設定 / New configuration

```toml
[server]
stop_on_last_detach = false
```

`true` 時，最後一個 client 離開（例如關掉視窗）就一併停止 server，不留背景程序。
預設 `false`，維持上游行為。

When `true`, the server stops once the last client detaches, instead of staying resident.
Defaults to `false`, preserving upstream behaviour.

## 4. 其他 / Misc

- `.gitignore` 加入 `/.zig-cache/`：`build.rs` 編譯 vendored libghostty-vt 時的本機快取。

---

## 分支策略 / Branching

| 分支 | 內容 |
|---|---|
| `master` | 與上游同步，**不含**本 fork 的修改 |
| `zh-tw` | 本 fork 的全部修改 |

升級上游時：

```bash
git fetch origin
git switch master && git merge --ff-only origin/master
git switch zh-tw && git rebase master
```

## Windows 建置注意 / Building on Windows

`build.rs` 會呼叫 Zig 編譯 vendored libghostty-vt。Zig 的路徑**不可含空白**，否則
`Step/Run.zig` 的 `assert(!isAbsolute(child_cwd_rel))` 會 panic。請把 Zig 裝在無空白的
路徑，並讓快取目錄也落在無空白路徑上：

```
ZIG=C:/Tools/zig-0.15.2/zig.exe
ZIG_GLOBAL_CACHE_DIR=D:/zig-cache
```

release 用 `cargo build --release`，不要加 `--target`（會整包重編）。
