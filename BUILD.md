# Build Guide - Moonraker Host Scanner

📋 **Complete guide for building and deploying Moonraker Host Scanner**

## 🛠️ Prerequisites

### Required Tools
- **Node.js** 18.0.0 or higher
- **pnpm** 8.0.0 or higher
- **Rust** 1.70.0 or higher (rustc, cargo)
- **Git** for version control

### Platform-Specific Dependencies

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install pnpm
npm install -g pnpm
```

#### Windows
```bash
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

# Install Rust
# Download from: https://rustup.rs/

# Install pnpm
npm install -g pnpm
```

#### Linux (Ubuntu/Debian)
```bash
# Install system dependencies
sudo apt update
sudo apt install -y build-essential curl git libssl-dev pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (if not already installed)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install pnpm
npm install -g pnpm
```

## 🚀 Development Setup

### 1. Clone Repository
```bash
git clone <repository-url>
cd MoonrakerHostScanner
```

### 2. Install Dependencies
```bash
# Install Node.js dependencies
pnpm install

# Verify Rust toolchain
cargo --version
rustc --version
```

### 3. Development Mode
```bash
# Start development server with hot reload
pnpm tauri:dev
```

## 📦 Building for Production

### Single Platform Build
```bash
# Build for current platform
pnpm tauri:build

# Build for specific platform
pnpm tauri:build --target x86_64-apple-darwin    # macOS Intel
pnpm tauri:build --target aarch64-apple-darwin   # macOS Apple Silicon
pnpm tauri:build --target x86_64-pc-windows-msvc # Windows
pnpm tauri:build --target x86_64-unknown-linux-gnu # Linux
```

### Multi-Platform Build Scripts
```bash
# Build for all platforms
pnpm build:all

# Platform-specific builds
pnpm build:macos
pnpm build:windows
pnpm build:linux
```

### Build Outputs
- **macOS**: `.app` bundle in `src-tauri/target/release/bundle/`
- **Windows**: `.exe` installer in `src-tauri/target/release/bundle/`
- **Linux**: `.AppImage` and `.deb` in `src-tauri/target/release/bundle/`

## 🔧 Build Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)
```json
{
  "build": {
    "beforeDevCommand": "pnpm build",
    "beforeBuildCommand": "pnpm build",
    "devPath": "http://localhost:3000",
    "distDir": "../out"
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "identifier": "com.moonraker.scanner",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### Next.js Configuration (`next.config.mjs`)
```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true
  }
}

export default nextConfig
```

## 🎯 Platform-Specific Builds

### macOS Build
```bash
# Intel Mac
pnpm tauri:build --target x86_64-apple-darwin

# Apple Silicon Mac
pnpm tauri:build --target aarch64-apple-darwin

# Universal Binary (both architectures)
pnpm tauri:build --target universal-apple-darwin
```

### Windows Build
```bash
# 64-bit Windows
pnpm tauri:build --target x86_64-pc-windows-msvc

# 32-bit Windows (if needed)
pnpm tauri:build --target i686-pc-windows-msvc
```

### Linux Build
```bash
# 64-bit Linux
pnpm tauri:build --target x86_64-unknown-linux-gnu

# ARM64 Linux (if needed)
pnpm tauri:build --target aarch64-unknown-linux-gnu
```

## 📱 Creating Installers

### macOS DMG
```bash
# After building for macOS
cd src-tauri/target/release/bundle/dmg
# DMG file will be created automatically
```

### Windows Installer
```bash
# After building for Windows
cd src-tauri/target/release/bundle/msi
# MSI installer will be created automatically
```

### Linux AppImage
```bash
# After building for Linux
cd src-tauri/target/release/bundle/appimage
# AppImage file will be created automatically
```

## 🔍 Troubleshooting

### Common Build Issues

#### Node.js/Pnpm Issues
```bash
# Clear cache and reinstall
rm -rf node_modules pnpm-lock.yaml
pnpm install

# Update pnpm
npm install -g pnpm@latest
```

#### Rust Build Issues
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### Tauri Build Issues
```bash
# Clear Tauri cache
rm -rf src-tauri/target

# Rebuild with verbose output
pnpm tauri:build --verbose
```

#### Platform-Specific Issues

**macOS:**
- Ensure Xcode Command Line Tools are installed
- Check code signing certificates if needed

**Windows:**
- Install Visual Studio Build Tools
- Ensure Windows SDK is installed

**Linux:**
- Install required system packages
- Check for missing libraries

### Debug Builds
```bash
# Development build with debug info
pnpm tauri:dev

# Debug production build
cargo build --debug
pnpm tauri:build --debug
```

## 🧪 Testing

### Unit Tests
```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (if configured)
pnpm test
```

### Integration Tests
```bash
# Test Tauri commands
pnpm tauri:dev
# Manually test all features in development mode
```

### Build Verification
```bash
# Verify build artifacts
ls -la src-tauri/target/release/bundle/

# Test application launch
./src-tauri/target/release/moonrakerhostscanner
```

## 📊 Build Performance

### Optimization Tips
- **Use release builds** for production
- **Enable parallel compilation** in Cargo.toml
- **Optimize Next.js build** with proper configuration
- **Use appropriate target** for your platform

### Build Times (Approximate)
- **Development build**: 30-60 seconds
- **Production build**: 2-5 minutes
- **Multi-platform build**: 10-20 minutes

## 🚀 Deployment

### GitHub Releases
```bash
# Tag release
git tag v0.1.0
git push origin v0.1.0

# Create GitHub release with build artifacts
# Upload files from src-tauri/target/release/bundle/
```

### Distribution
- **macOS**: Upload `.dmg` file
- **Windows**: Upload `.msi` installer
- **Linux**: Upload `.AppImage` and `.deb` packages

## 📋 Build Checklist

### Pre-Build
- [ ] All dependencies installed
- [ ] Code compiles without errors
- [ ] Tests pass
- [ ] Version numbers updated
- [ ] Changelog updated

### Build Process
- [ ] Frontend builds successfully
- [ ] Rust backend compiles
- [ ] Tauri bundle created
- [ ] Platform-specific installers generated
- [ ] Application launches correctly

### Post-Build
- [ ] Test application functionality
- [ ] Verify all features work
- [ ] Check file sizes are reasonable
- [ ] Create release notes
- [ ] Upload to distribution platform

## 🔄 Continuous Integration

### GitHub Actions Workflow
```yaml
# .github/workflows/build.yml
name: Build and Release

on:
  push:
    tags: ['v*']

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [macos-latest, windows-latest, ubuntu-latest]
    
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '18'
      - uses: actions/setup-rust@v1
        with:
          rust-version: '1.70'
      - run: npm install -g pnpm
      - run: pnpm install
      - run: pnpm tauri:build
      - uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.os }}-build
          path: src-tauri/target/release/bundle/
```

---

**For more detailed information, see the [Tauri documentation](https://tauri.app/docs/get-started/setup/) and [Next.js documentation](https://nextjs.org/docs).**
