# Herdr Windows 安裝與原始碼編譯完整紀錄

> 環境：Windows 11 / PowerShell  
> 專案：`https://github.com/herdrdev/herdr.git`  
> 本次成功編譯版本：Herdr `0.8.2`  
> Rust target：Windows MSVC  
> Zig：`0.15.2`

---

## 1. 安裝 Git for Windows

如果 Windows 尚未安裝 Git：

```powershell
winget install --id Git.Git -e
```

重新開啟 PowerShell 後確認：

```powershell
git --version
where.exe git
```

下載 Herdr：

```powershell
cd D:\github
git clone https://github.com/herdrdev/herdr.git
cd herdr
```

---

## 2. 安裝 Rust / Cargo

Herdr 是 Rust 專案，因此需要 Rust 工具鏈與 Cargo。

```powershell
winget install Rustlang.Rustup
```

安裝後關閉並重新開啟 PowerShell，再確認：

```powershell
rustc --version
cargo --version
rustup show
```

接著可以先嘗試：

```powershell
cargo build --release
```

### 曾遇到的錯誤

```text
error: linker `link.exe` not found

note: the msvc targets depend on the msvc linker but `link.exe` was not found
```

這代表 Rust 已經安裝，但 Windows 的 MSVC linker 尚未安裝。

---

## 3. 安裝 Visual Studio Build Tools 2022

安裝：

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

然後開啟：

```text
Visual Studio Installer
→ Visual Studio Build Tools 2022
→ 修改
```

至少勾選：

```text
✅ 使用 C++ 的桌面開發
```

其中至少需要：

```text
✅ MSVC v143 - VS 2022 C++ x64/x86 建置工具
✅ Windows 11 SDK
✅ 適用於 Windows 的 C++ CMake 工具
```

其他依個人需求可選：

```text
✅ .NET 桌面建置工具
✅ WinUI 應用程式開發建置工具
✅ 資料存放區及處理建置工具
△ .NET MAUI
```

Herdr 本身最關鍵的是 **使用 C++ 的桌面開發**。

安裝完成後重新開 PowerShell。

如果一般 PowerShell 找不到 `link.exe`，可從開始選單開：

```text
Developer PowerShell for VS 2022
```

或：

```text
x64 Native Tools Command Prompt for VS 2022
```

確認：

```powershell
where.exe link
```

---

## 4. Herdr 需要 Zig 0.15.2

再次執行：

```powershell
cargo build --release
```

曾遇到：

```text
zig executable not found
Building the vendored libghostty-vt requires Zig 0.15.2
```

查 Winget Zig 版本：

```powershell
winget search zig
winget show zig.zig --versions
```

本次可看到：

```text
0.16.0
0.15.2
0.15.1
...
```

Herdr build 明確需要 Zig `0.15.2`，因此安裝指定版本：

```powershell
winget install zig.zig --version 0.15.2
```

確認：

```powershell
zig version
```

應得到：

```text
0.15.2
```

查看 Zig 環境：

```powershell
zig env
```

本次 Winget 安裝位置：

```text
C:\Users\<yourname>\AppData\Local\Microsoft\WinGet\Packages\
zig.zig_Microsoft.Winget.Source_8wekyb3d8bbwe\
zig-x86_64-windows-0.15.2\zig.exe
```

原本 global cache：

```text
C:\Users\<yourname>\AppData\Local\zig
```

---

## 5. 將 Zig 移到較乾淨的固定路徑

為避免 Winget 路徑過長、使用者名稱有空白，以及 Zig Windows path handling 問題，本次將 Zig 複製到：

```text
C:\Tools\zig-0.15.2
```

建立資料夾：

```powershell
New-Item -ItemType Directory -Force C:\Tools\zig-0.15.2
```

確認原 Zig 路徑：

```powershell
(Get-Command zig).Source
```

複製整個 Zig 目錄：

```powershell
Copy-Item `
"C:\Users\<yourname>\AppData\Local\Microsoft\WinGet\Packages\zig.zig_Microsoft.Winget.Source_8wekyb3d8bbwe\zig-x86_64-windows-0.15.2\*" `
"C:\Tools\zig-0.15.2\" `
-Recurse -Force
```

確認新位置：

```powershell
C:\Tools\zig-0.15.2\zig.exe version
```

應得到：

```text
0.15.2
```

---

## 6. 指定 Zig 與 Cache 路徑

建立 Zig cache：

```powershell
New-Item -ItemType Directory -Force "D:\zig-cache"
New-Item -ItemType Directory -Force "D:\github\herdr\.zig-cache"
```

在目前 PowerShell session 設定：

```powershell
$env:ZIG = "C:\Tools\zig-0.15.2\zig.exe"
$env:ZIG_GLOBAL_CACHE_DIR = "D:\zig-cache"
$env:ZIG_LOCAL_CACHE_DIR = "D:\github\herdr\.zig-cache"
```

確認：

```powershell
$env:ZIG
$env:ZIG_GLOBAL_CACHE_DIR
$env:ZIG_LOCAL_CACHE_DIR

& $env:ZIG version
```

---

## 7. Avira 誤判 Wuffs 測試 JPEG

Build 過程曾出現：

```text
error: unable to hash
'wuffs-0.4.0-alpha.9\test\data\artificial-jpeg\
hippopotamus-bad-comment-length.jpeg': AccessDenied
```

Avira 顯示：

```text
威脅名稱：EXP/MS04-028.JPEG.A
類型：惡意探索程式碼
狀態：移至隔離區
```

這個檔案來自 Wuffs 的 parser 測試資料；它是刻意設計的異常 JPEG 測試 fixture，因此容易被防毒誤判。

### Avira 排除資料夾

到：

```text
Avira
→ 設定
→ 安全性
→ 病毒掃描
→ 排除的資料夾
```

加入：

```text
C:\Users\<yourname>\AppData\Local\zig
D:\zig-cache
D:\github\herdr\.zig-cache
```

如果仍有 target build 中間檔被攔，可再考慮：

```text
D:\github\herdr\target
```

不建議直接排除整個 `D:\github`。

---

## 8. 清理舊 Cache 與 Build

每次調整 Zig / cache / 防毒排除後，可先清掉舊內容：

```powershell
Remove-Item "D:\zig-cache" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "D:\github\herdr\.zig-cache" -Recurse -Force -ErrorAction SilentlyContinue
```

如果要重建目錄：

```powershell
New-Item -ItemType Directory -Force "D:\zig-cache"
New-Item -ItemType Directory -Force "D:\github\herdr\.zig-cache"
```

清 Cargo：

```powershell
cargo clean
```

---

## 9. 編譯 Herdr

進到 Herdr：

```powershell
cd D:\github\herdr
```

確認 Zig 環境：

```powershell
$env:ZIG = "C:\Tools\zig-0.15.2\zig.exe"
$env:ZIG_GLOBAL_CACHE_DIR = "D:\zig-cache"
$env:ZIG_LOCAL_CACHE_DIR = "D:\github\herdr\.zig-cache"
```

開始 release build：

```powershell
cargo build --release
```

本次最後成功訊息：

```text
Finished `release` profile [optimized] target(s) in 4m 33s
```

代表編譯完成。

---

## 10. Herdr 編譯完成位置

Release 執行檔：

```text
D:\github\herdr\target\release\herdr.exe
```

確認：

```powershell
.\target\release\herdr.exe --version
```

直接啟動：

```powershell
.\target\release\herdr.exe
```

---

## 11. 將 Herdr 移到固定工具目錄

建議不要日常直接使用 `target\release` 裡的 binary。

建立：

```powershell
New-Item -ItemType Directory -Force C:\Tools\herdr
```

複製：

```powershell
Copy-Item `
"D:\github\herdr\target\release\herdr.exe" `
"C:\Tools\herdr\herdr.exe" `
-Force
```

確認：

```powershell
C:\Tools\herdr\herdr.exe --version
```

---

## 12. 將 Herdr 加入使用者 PATH

先查看目前使用者 PATH：

```powershell
[Environment]::GetEnvironmentVariable("Path", "User")
```

加入：

```powershell
$oldPath = [Environment]::GetEnvironmentVariable("Path", "User")

if (($oldPath -split ';') -notcontains 'C:\Tools\herdr') {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$oldPath;C:\Tools\herdr",
        "User"
    )
}
```

重新開啟 PowerShell 後：

```powershell
where.exe herdr
herdr --version
```

理想位置：

```text
C:\Tools\herdr\herdr.exe
```

之後在任何專案目錄都可以直接：

```powershell
herdr
```

---

# 完整工具鏈整理

最後 Windows 上的 Herdr 原始碼編譯環境：

```text
Git for Windows
│
├─ Rust / rustup
│  ├─ rustc
│  └─ cargo
│
├─ Visual Studio Build Tools 2022
│  ├─ MSVC v143 x64/x86
│  ├─ link.exe
│  ├─ Windows 11 SDK
│  └─ CMake Tools
│
├─ Zig 0.15.2
│  └─ C:\Tools\zig-0.15.2\zig.exe
│
├─ Zig Global Cache
│  └─ D:\zig-cache
│
├─ Zig Local Cache
│  └─ D:\github\herdr\.zig-cache
│
├─ Herdr source
│  └─ D:\github\herdr
│
└─ Herdr installed binary
   └─ C:\Tools\herdr\herdr.exe
```

---

# 本次主要錯誤與解法

## `cargo` 找不到

```text
cargo : 無法辨識 'cargo'
```

解法：

```powershell
winget install Rustlang.Rustup
```

---

## `link.exe not found`

```text
error: linker `link.exe` not found
```

解法：

```text
Visual Studio Installer
→ Visual Studio Build Tools 2022
→ 使用 C++ 的桌面開發
```

至少勾：

```text
MSVC v143
Windows 11 SDK
C++ CMake Tools
```

---

## `zig executable not found`

```text
zig executable not found
Building the vendored libghostty-vt requires Zig 0.15.2
```

解法：

```powershell
winget install zig.zig --version 0.15.2
```

---

## Zig Wuffs JPEG `AccessDenied`

```text
hippopotamus-bad-comment-length.jpeg: AccessDenied
```

原因：

```text
Avira 將 Wuffs 測試 JPEG 判定成 EXP/MS04-028.JPEG.A
並移至隔離區
```

解法：

```text
在 Avira 排除 Zig cache
```

例如：

```text
D:\zig-cache
D:\github\herdr\.zig-cache
```

然後：

```powershell
cargo clean
cargo build --release
```

---

## Zig `convertPathArg` panic

曾出現：

```text
assert(!std.fs.path.isAbsolute(child_cwd_rel))
```

為降低 Windows 路徑問題，本次把 Zig 從：

```text
C:\Users\<yourname>\AppData\Local\Microsoft\WinGet\Packages\...
```

另外複製到：

```text
C:\Tools\zig-0.15.2
```

並指定：

```powershell
$env:ZIG = "C:\Tools\zig-0.15.2\zig.exe"
```

Cache 同樣移至：

```text
D:\zig-cache
D:\github\herdr\.zig-cache
```

---

# 日後更新 Herdr

進 repo：

```powershell
cd D:\github\herdr
```

拉最新版：

```powershell
git pull
```

設定 build environment：

```powershell
$env:ZIG = "C:\Tools\zig-0.15.2\zig.exe"
$env:ZIG_GLOBAL_CACHE_DIR = "D:\zig-cache"
$env:ZIG_LOCAL_CACHE_DIR = "D:\github\herdr\.zig-cache"
```

重新編譯：

```powershell
cargo build --release
```

更新正式使用的 binary：

```powershell
Copy-Item `
".\target\release\herdr.exe" `
"C:\Tools\herdr\herdr.exe" `
-Force
```

確認：

```powershell
herdr --version
```

> 注意：未來 Herdr 若更新 `build.rs` 並要求不同 Zig 版本，請以專案當下要求為準，不要固定假設永遠使用 Zig 0.15.2。

---

# 快速重建指令摘要

```powershell
# Git
winget install --id Git.Git -e

# Rust
winget install Rustlang.Rustup

# Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# Zig 0.15.2
winget install zig.zig --version 0.15.2

# Clone
cd D:\github
git clone https://github.com/herdrdev/herdr.git
cd herdr

# Zig 固定位置（完成複製後）
$env:ZIG = "C:\Tools\zig-0.15.2\zig.exe"

# Cache
New-Item -ItemType Directory -Force "D:\zig-cache"
New-Item -ItemType Directory -Force "D:\github\herdr\.zig-cache"

$env:ZIG_GLOBAL_CACHE_DIR = "D:\zig-cache"
$env:ZIG_LOCAL_CACHE_DIR = "D:\github\herdr\.zig-cache"

# Build
cargo clean
cargo build --release

# Install Herdr
New-Item -ItemType Directory -Force C:\Tools\herdr
Copy-Item ".\target\release\herdr.exe" "C:\Tools\herdr\herdr.exe" -Force

# Run
C:\Tools\herdr\herdr.exe --version
```
