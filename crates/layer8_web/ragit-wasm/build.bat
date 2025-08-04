@echo off
echo Building RAGIT WASM...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Installing wasm-pack...
    cargo install wasm-pack
)

REM Build the WASM package
wasm-pack build --target web --out-dir js/pkg

REM Copy extension files
echo Preparing extension files...
copy js\manifest.json js\pkg\
copy js\popup.html js\pkg\
copy js\content-script.js js\pkg\

echo Build complete! Extension files are in js/pkg/
echo To install:
echo 1. Open Chrome and go to chrome://extensions/
echo 2. Enable Developer mode
echo 3. Click 'Load unpacked' and select the js/pkg/ directory