# Moonraker Host Scanner

🚀 **Desktop application for scanning and managing Moonraker 3D printer hosts**

A modern Tauri-based desktop application that automatically discovers Moonraker-enabled 3D printers on your network, provides real-time status monitoring, and offers direct control over your printers.

### Для запуска на MacOS необходимо
```
xattr -rd com.apple.quarantine /Applications/Moonraker\ Host\ Scanner.app
```

## ✨ Features

### 🔍 **Network Discovery**
- **Automatic scanning** of local network subnets
- **Port-based detection** (port 7125) for efficient discovery
- **Real-time host detection** with configurable scan intervals
- **Custom subnet configuration** for targeted scanning

### 📊 **Printer Monitoring**
- **Real-time status updates** every second for known hosts
- **Moonraker API integration** for accurate printer state detection
- **Status flags support**: printing, paused, cancelling, error, standby, offline
- **Printer information display**: version, Klippy state, hostname

### 🎮 **Printer Control**
- **Start/Pause/Stop** print jobs
- **Emergency stop** functionality
- **Direct API integration** with Moonraker endpoints
- **Visual feedback** with button animations

### 🖥️ **Webcam Integration**
- **Live webcam streaming** in modal windows
- **Image manipulation**: rotate, flip horizontal/vertical
- **Direct stream display** (no browser redirection)

### 🔗 **Quick Access**
- **One-click browser access** to printer web interface
- **SSH terminal integration** for direct host access
- **Cross-platform support** (macOS, Windows, Linux)

### 🔔 **Smart Notifications**
- **System notifications** for status changes
- **Configurable notification settings** per status type
- **Multi-language support** for notifications

### 🌍 **Internationalization**
- **English, Russian, German** language support
- **Modular translation system** with separate language files
- **Dynamic language switching**

### ⚙️ **Settings & Persistence**
- **Custom hostname support** with rename functionality
- **Persistent settings** across application restarts
- **Configurable scan intervals** and notification preferences
- **Local storage** for user preferences

## 🛠️ Technology Stack

- **Frontend**: Next.js 15, React 18, TypeScript
- **Backend**: Rust, Tauri 2
- **UI Components**: shadcn/ui, Tailwind CSS
- **Network**: tokio, reqwest, ipnetwork
- **Notifications**: notify-rust
- **Build System**: pnpm, Cargo

## 📦 Installation

### Prerequisites
- **Node.js** 18+ and **pnpm**
- **Rust** toolchain (rustc, cargo)
- **Platform-specific dependencies** (see BUILD.md)

### Development Setup
```bash
# Clone the repository
git clone <repository-url>
cd MoonrakerHostScanner

# Install dependencies
pnpm install

# Start development server
pnpm tauri:dev
```

### Production Build
```bash
# Build for current platform
pnpm tauri:build

# Build for specific platform
pnpm tauri:build --target x86_64-apple-darwin  # macOS
pnpm tauri:build --target x86_64-pc-windows-msvc  # Windows
pnpm tauri:build --target x86_64-unknown-linux-gnu  # Linux
```

## 🚀 Usage

### Initial Setup
1. **Launch the application**
2. **Configure network subnets** in settings (default: auto-detection)
3. **Enable desired notifications** for printer status changes
4. **Start network scanning**

### Network Scanning
- **Automatic scanning** runs at configured intervals
- **Status updates** occur every second for online hosts
- **New hosts** are automatically discovered and added
- **Offline hosts** are marked but not removed

### Printer Management
- **Click hostname** to open printer web interface
- **Use control buttons** for print job management
- **Click webcam button** for live stream viewing
- **Click SSH button** for terminal access

### Customization
- **Rename hosts** by clicking the edit icon
- **Reset hostnames** to original values
- **Configure notification preferences** per status type
- **Adjust scan intervals** for your network

## 🔧 Configuration

### Network Settings
- **Subnet configuration**: Add custom subnets for scanning
- **Scan interval**: Configure automatic scan frequency
- **Port detection**: Moonraker API port (default: 7125)

### Notification Settings
- **Status-based notifications**: Enable/disable per status type
- **System integration**: Native platform notifications
- **Multi-language support**: Localized notification messages

### Display Settings
- **Language selection**: English, Russian, German
- **Theme support**: Light/dark mode (system-based)
- **UI customization**: Responsive design for all screen sizes

## 📁 Project Structure

```
MoonrakerHostScanner/
├── src/                    # Frontend source code
│   ├── app/               # Next.js app directory
│   ├── components/        # React components
│   │   ├── ui/           # shadcn/ui components
│   │   └── network-scanner.tsx
│   ├── lib/              # Utilities and translations
│   │   └── translations/ # Language files
│   └── hooks/            # Custom React hooks
├── src-tauri/            # Rust backend
│   ├── src/              # Rust source code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── public/               # Static assets
└── docs/                 # Documentation
```

## 🌐 API Integration

### Moonraker Endpoints
- `GET /server/info` - Server information
- `GET /api/printer` - Printer status and flags
- `POST /printer/print/start` - Start print job
- `POST /printer/print/pause` - Pause print job
- `POST /printer/print/cancel` - Cancel print job
- `POST /printer/emergency_stop` - Emergency stop
- `GET /webcam/?action=stream` - Webcam stream

### Status Flags
- `operational` - Printer is operational
- `paused` - Print job is paused
- `printing` - Currently printing
- `cancelling` - Cancelling print job
- `error` - Printer error state
- `ready` - Printer ready (mapped to standby)

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

### Development Workflow
1. **Fork the repository**
2. **Create feature branch**: `git checkout -b feature/amazing-feature`
3. **Make changes** and test thoroughly
4. **Commit changes**: `git commit -m 'Add amazing feature'`
5. **Push to branch**: `git push origin feature/amazing-feature`
6. **Open Pull Request**

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

### Common Issues
- **Network scanning issues**: Check firewall settings and subnet configuration
- **Notification problems**: Verify system notification permissions
- **Build errors**: Ensure all prerequisites are installed (see BUILD.md)

### Getting Help
- **Check documentation**: Review BUILD.md and CONTRIBUTING.md
- **Search issues**: Look for similar problems in GitHub issues
- **Create issue**: Provide detailed information about your problem

## 🔄 Version History

### v0.1.0 (Current)
- ✅ Network discovery and host scanning
- ✅ Real-time status monitoring
- ✅ Printer control integration
- ✅ Webcam streaming support
- ✅ System notifications
- ✅ Multi-language support
- ✅ Cross-platform compatibility

---

**Built with ❤️ using Tauri and Next.js**
