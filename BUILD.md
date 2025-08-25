# Build Guide

This document provides detailed instructions for building the Moonraker Host Scanner application for different platforms.

## Prerequisites

### Required Tools

- **Node.js** 18+ and **pnpm**
- **Rust** toolchain (latest stable)
- **Tauri CLI**: `pnpm add -g @tauri-apps/cli`

### Platform-Specific Requirements

#### Windows
- Visual Studio Build Tools 2019 or later
- Windows 10/11 SDK
- WebView2 Runtime

#### macOS
- Xcode Command Line Tools
- macOS 10.15+ (for building)
- Apple Developer Account (for distribution)

#### Linux
- `build-essential` package
- `libwebkit2gtk-4.0-dev` and `libgtk-3-dev`
- `libayatana-appindicator3-dev` (for system tray)

## Development Setup

1. **Clone and install dependencies**:
```bash
git clone https://github.com/yourusername/moonraker-host-scanner.git
cd moonraker-host-scanner
pnpm install
```

2. **Verify Rust toolchain**:
```bash
rustup show
cargo --version
```

3. **Test development build**:
```bash
pnpm tauri:dev
```

## Building for Production

### Single Platform

```bash
# Build for current platform
pnpm tauri:build

# Build for specific platform
pnpm tauri build --target x86_64-pc-windows-msvc    # Windows x64
pnpm tauri build --target x86_64-apple-darwin       # macOS Intel
pnpm tauri build --target aarch64-apple-darwin      # macOS Apple Silicon
pnpm tauri build --target x86_64-unknown-linux-gnu  # Linux x64
```

### All Platforms (Cross-Compilation)

```bash
# Build for all supported platforms
pnpm tauri build --target all
```

**Note**: Cross-compilation requires additional setup for each target platform.

## Build Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

Key build settings:
```json
{
  "build": {
    "frontendDist": "../out",
    "distDir": "../out"
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "identifier": "com.tormyhseviv.moonrakerhostscanner",
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

Static export configuration:
```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  distDir: 'out',
  assetPrefix: '/',
  images: {
    unoptimized: true
  }
}
```

## Platform-Specific Build Instructions

### Windows

1. **Install Visual Studio Build Tools**:
   - Download from Microsoft Visual Studio
   - Install C++ build tools and Windows 10/11 SDK

2. **Install WebView2 Runtime**:
   - Required for Tauri applications
   - Download from Microsoft

3. **Build**:
```bash
pnpm tauri build --target x86_64-pc-windows-msvc
```

**Output**: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/`

### macOS

1. **Install Xcode Command Line Tools**:
```bash
xcode-select --install
```

2. **Build for Intel Macs**:
```bash
pnpm tauri build --target x86_64-apple-darwin
```

3. **Build for Apple Silicon**:
```bash
pnpm tauri build --target aarch64-apple-darwin
```

4. **Create DMG files** (after building):
```bash
# Create DMG for Apple Silicon
./scripts/create-dmg.sh aarch64-apple-darwin

# Create DMG for Intel Macs
./scripts/create-dmg.sh x86_64-apple-darwin
```

5. **Universal Binary** (both architectures):
```bash
# Build both targets
pnpm tauri build --target x86_64-apple-darwin
pnpm tauri build --target aarch64-apple-darwin

# Create universal binary using lipo
lipo -create \
  src-tauri/target/x86_64-apple-darwin/release/bundle/macos/moonraker-host-scanner \
  src-tauri/target/aarch64-apple-darwin/release/bundle/macos/moonraker-host-scanner \
  -output moonraker-host-scanner-universal
```

**Output**: 
- App bundles: `src-tauri/target/*/release/bundle/macos/`
- DMG files: `src-tauri/target/*/release/bundle/dmg/`

**Note**: DMG creation is done manually using the provided script due to Tauri 2 compatibility issues.

### Linux

1. **Install dependencies** (Ubuntu/Debian):
```bash
sudo apt update
sudo apt install build-essential libwebkit2gtk-4.0-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

2. **Build**:
```bash
pnpm tauri build --target x86_64-unknown-linux-gnu
```

**Output**: `src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/`

## Continuous Integration

### GitHub Actions Workflow

Create `.github/workflows/build.yml`:

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        target:
          - x86_64-unknown-linux-gnu
          - x86_64-pc-windows-msvc
          - x86_64-apple-darwin
          - aarch64-apple-darwin

    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
          
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8
          
      - name: Install dependencies
        run: pnpm install
        
      - name: Build application
        run: pnpm tauri build --target ${{ matrix.target }}
        
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: moonraker-host-scanner-${{ matrix.target }}
          path: src-tauri/target/${{ matrix.target }}/release/bundle/
```

## Distribution

### Windows
- **MSI Installer**: Generated automatically by Tauri
- **Portable**: `.exe` file in bundle directory
- **Code Signing**: Recommended for distribution

### macOS
- **DMG**: Generated automatically by Tauri
- **App Bundle**: `.app` file in bundle directory
- **Notarization**: Required for distribution outside App Store

### Linux
- **AppImage**: Generated automatically by Tauri
- **Deb Package**: Can be created using `cargo deb`
- **RPM Package**: Can be created using `cargo rpm`

## Troubleshooting

### Common Build Issues

1. **Rust toolchain issues**:
```bash
rustup update
rustup default stable
```

2. **Node.js version conflicts**:
```bash
node --version  # Should be 18+
pnpm --version  # Should be 8+
```

3. **Tauri CLI issues**:
```bash
pnpm remove -g @tauri-apps/cli
pnpm add -g @tauri-apps/cli@latest
```

4. **Platform-specific dependencies**:
   - Windows: Ensure Visual Studio Build Tools are installed
   - macOS: Ensure Xcode Command Line Tools are installed
   - Linux: Install required system packages

### Performance Optimization

1. **Enable Rust optimizations**:
```toml
# src-tauri/Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = 'abort'
```

2. **Optimize Next.js build**:
```javascript
// next.config.mjs
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  distDir: 'out',
  assetPrefix: '/',
  images: {
    unoptimized: true
  },
  experimental: {
    optimizeCss: true
  }
}
```

## Version Management

### Semantic Versioning
- **Major**: Breaking changes
- **Minor**: New features
- **Patch**: Bug fixes

### Release Process
1. Update version in `package.json` and `src-tauri/Cargo.toml`
2. Create git tag: `git tag v1.0.0`
3. Push tag: `git push origin v1.0.0`
4. GitHub Actions will automatically build and release

## Security Considerations

1. **Code Signing**: Sign all releases for Windows and macOS
2. **Notarization**: Notarize macOS releases
3. **Dependency Updates**: Regularly update dependencies
4. **Security Audits**: Run `pnpm audit` and `cargo audit`

## Support

For build-related issues:
- Check [Tauri Documentation](https://tauri.app/docs/get-started/setup)
- Review [GitHub Issues](https://github.com/yourusername/moonraker-host-scanner/issues)
- Join [Tauri Discord](https://discord.gg/tauri)
