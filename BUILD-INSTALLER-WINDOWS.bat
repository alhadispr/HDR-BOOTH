@echo off
setlocal
cd /d "%~dp0"
echo ============================================================
echo HDR BOOTH Professional - Windows Installer Builder
echo ============================================================
where node >nul 2>nul || (echo [ERROR] Install Node.js 22 first. & pause & exit /b 1)
where npm >nul 2>nul || (echo [ERROR] npm is missing. & pause & exit /b 1)
where cargo >nul 2>nul || (echo [ERROR] Install Rust stable with MSVC target first. & pause & exit /b 1)
where rustc >nul 2>nul || (echo [ERROR] rustc is missing. & pause & exit /b 1)
echo Installing frontend dependencies...
call npm install
if errorlevel 1 (echo [ERROR] npm install failed. & pause & exit /b 1)
echo Building HDR BOOTH Windows NSIS installer...
call npm run tauri -- build --bundles nsis
if errorlevel 1 (echo [ERROR] Windows build failed. & pause & exit /b 1)
echo BUILD COMPLETE. Installer location:
echo src-tauri\target\release\bundle\nsis\
explorer "%~dp0src-tauri\target\release\bundle\nsis"
pause
