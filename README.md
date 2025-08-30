# 🖨️ MoonrakerHostScanner

A modern, cross-platform desktop application for discovering, monitoring, and controlling Moonraker-enabled 3D printers on your network.

## ✨ Features

- **🔍 Network Discovery** - Automatically scan and discover Moonraker hosts
- **📊 Real-time Monitoring** - Live status updates with configurable refresh intervals
- **🎮 Printer Control** - Start, pause, stop, and emergency stop functionality
- **🔗 SSH Integration** - Direct terminal access to hosts
- **🌐 Browser Integration** - Quick access to web interfaces
- **📷 Webcam Support** - Stream printer webcams
- **🔔 Smart Notifications** - Configurable system notifications for status changes
- **🌍 Multi-language** - English and Russian support
- **🎨 Theme Support** - Light, dark, and system themes
- **⚙️ Persistent Settings** - All settings saved locally

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ 
- **pnpm** (recommended) or npm
- **Rust** 1.70+ (for Tauri)
- **macOS** / **Windows** / **Linux**

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd MoonrakerHostScanner
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Run in development mode**
   ```bash
   pnpm tauri:dev
   ```

4. **Build for production**
   ```bash
   pnpm tauri:build
   ```

## 🏗️ Architecture

### Frontend (React + TypeScript)
- **Modern React 18** with hooks and functional components
- **TypeScript** for type safety
- **Tailwind CSS** for styling
- **Shadcn/ui** for UI components
- **Custom hooks** for state management and business logic

### Backend (Rust + Tauri)
- **Tauri 2.0** for cross-platform desktop capabilities
- **Rust** for high-performance system operations
- **Async/await** for non-blocking operations
- **Error handling** with graceful fallbacks

### Key Components

```
src/
├── hooks/                    # Custom React hooks
│   ├── useSettings.ts       # Settings management
│   ├── useHosts.ts          # Host state management
│   ├── useTheme.ts          # Theme management
│   ├── useLanguage.ts       # Language management
│   ├── useNetworkScanner.ts # Network scanning
│   ├── useHostStatus.ts     # Host status monitoring
│   └── useNotifications.ts  # Notification system
├── components/              # React components
│   ├── network-scanner.tsx  # Main application component
│   ├── host-table.tsx       # Host table component
│   └── ui/                  # Reusable UI components
├── lib/                     # Utilities and helpers
│   ├── tauri.ts            # Tauri API wrapper
│   ├── i18n.ts             # Internationalization
│   └── utils.ts            # General utilities
└── app/                    # Next.js app structure
```

## 🔧 Configuration

### Network Settings
- Add custom subnets for scanning
- Configure scan intervals
- Enable/disable specific networks

### Notification Settings
- Configure notifications for different printer states
- Customize notification preferences
- Test notification system

### UI Settings
- Choose theme (light/dark/system)
- Select language (English/Russian)
- Configure SSH default user

## 📱 Usage

### Adding Hosts
1. **Configure subnets** in Settings
2. **Start network scan**
3. **Review discovered hosts**
4. **Customize hostnames** if needed

### Monitoring
- **Automatic status updates** every 3 seconds (configurable)
- **Real-time status indicators**
- **Failed connection tracking**
- **Smart offline detection**

### Printer Control
- **Start printing** from prepared files
- **Pause/resume** active prints
- **Stop printing** safely
- **Emergency stop** for critical situations

### System Integration
- **SSH terminal** access
- **Web browser** integration
- **Webcam streaming**
- **System notifications**

## 🛠️ Development

### Project Structure
```
MoonrakerHostScanner/
├── src/                    # Frontend source code
├── src-tauri/             # Backend source code
├── docs/                  # Documentation
├── scripts/               # Build and utility scripts
└── public/                # Static assets
```

### Key Technologies
- **Frontend**: React 18, TypeScript, Tailwind CSS
- **Backend**: Rust, Tauri 2.0
- **Build**: Vite, Next.js
- **Package Manager**: pnpm

### Development Commands
```bash
# Development
pnpm tauri:dev          # Start development server
pnpm tauri:build        # Build for production
pnpm tauri:preview      # Preview production build

# Code quality
pnpm lint               # Run ESLint
pnpm type-check         # Run TypeScript checks
pnpm format             # Format code with Prettier

# Version management
pnpm version:update     # Update version in all files
pnpm release            # Create release with git tag
```

## 🧪 Testing

### Manual Testing
1. **Network scanning** - Verify host discovery
2. **Status monitoring** - Check automatic updates
3. **Printer control** - Test all control buttons
4. **Notifications** - Verify notification system
5. **System integration** - Test SSH, browser, webcam

### Automated Testing
- **Unit tests** for utility functions
- **Integration tests** for Tauri commands
- **E2E tests** for critical user flows

## 🚀 Performance Optimizations

### Recent Improvements
- **Custom hooks** for better state management
- **Memoized computations** for expensive operations
- **Optimized re-renders** with React.memo
- **Efficient localStorage** operations
- **Reduced bundle size** with tree shaking

### Performance Features
- **Lazy loading** of components
- **Debounced network operations**
- **Optimized status polling**
- **Memory leak prevention**

## 📦 Distribution

### Build Targets
- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image
- **Linux**: `.AppImage` and `.deb` packages

### Release Process
1. **Update version** with `pnpm version:update <version>`
2. **Create release** with `pnpm release <version> [message]`
3. **Build all targets** with `pnpm tauri:build`
4. **Create GitHub release** with assets
5. **Update documentation** and changelog

### Version Management
- **Automatic version sync** across all files
- **Semantic versioning** (MAJOR.MINOR.PATCH)
- **Git tag integration** for releases
- **See [VERSIONING.md](./VERSIONING.md)** for detailed instructions

## 🤝 Contributing

### Development Setup
1. **Fork the repository**
2. **Create feature branch**
3. **Make changes** following coding standards
4. **Test thoroughly**
5. **Submit pull request**

### Coding Standards
- **TypeScript** for type safety
- **ESLint** for code quality
- **Prettier** for formatting
- **Conventional commits** for commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Moonraker** team for the excellent API
- **Tauri** team for the amazing desktop framework
- **Shadcn/ui** for beautiful UI components
- **Vercel** for Next.js and Vite

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-repo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-repo/discussions)
- **Documentation**: [Wiki](https://github.com/your-repo/wiki)

---

**Made with ❤️ for the 3D printing community**
