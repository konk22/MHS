# Moonraker Host Scanner

A modern desktop application for discovering and managing Moonraker-enabled 3D printers on your network. Built with Tauri, Next.js, and TypeScript.

## Features

- 🔍 **Network Scanning**: Fast discovery of Moonraker hosts on your network
- 🖨️ **Printer Control**: Start, pause, cancel, and emergency stop print jobs
- 📹 **Webcam Streaming**: View live webcam feeds with rotation and flip controls
- 🌐 **Browser Integration**: Open printer web interfaces directly
- 🔧 **SSH Access**: Quick terminal access to printer hosts
- 💾 **Settings Persistence**: Save and restore application settings
- 🌍 **Multi-language**: English and Russian support
- 🎨 **Modern UI**: Beautiful, responsive interface with dark/light themes

## Screenshots

![Main Interface](docs/screenshots/main.png)
![Webcam Modal](docs/screenshots/webcam.png)

## Prerequisites

- **Node.js** 18+ and **pnpm**
- **Rust** toolchain (for Tauri)
- **Moonraker** running on your 3D printers (port 7125)

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/moonraker-host-scanner.git
cd moonraker-host-scanner
```

2. Install dependencies:
```bash
pnpm install
```

3. Run in development mode:
```bash
pnpm tauri:dev
```

### Building

Build for your platform:
```bash
pnpm tauri:build
```

## Usage

1. **Configure Network Settings**: Set your network range and scan options
2. **Scan Network**: Click "Scan" to discover Moonraker hosts
3. **Manage Printers**: Use the control buttons for print operations
4. **View Webcam**: Click the webcam button to see live streams
5. **Access Hosts**: Click IP addresses to open in browser or SSH

## API Integration

The application integrates with Moonraker's REST API:

- `GET /server/info` - Host discovery and info
- `POST /printer/print/start` - Start print job
- `POST /printer/print/pause` - Pause print job  
- `POST /printer/print/cancel` - Cancel print job
- `POST /printer/emergency_stop` - Emergency stop
- `GET /webcam/?action=stream` - Webcam stream

## Development

### Project Structure

```
MoonrakerHostScanner/
├── src/                    # Next.js frontend
│   ├── app/               # App router pages
│   ├── components/        # React components
│   ├── lib/              # Utilities and i18n
│   └── hooks/            # Custom React hooks
├── src-tauri/            # Rust backend
│   ├── src/              # Rust source code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
└── docs/                 # Documentation
```

### Available Scripts

- `pnpm dev` - Start Next.js development server
- `pnpm build` - Build Next.js for production
- `pnpm tauri:dev` - Start Tauri development mode
- `pnpm tauri:build` - Build Tauri application for current platform
- `pnpm build:macos` - Build for macOS (Intel + Apple Silicon)
- `pnpm build:windows` - Build for Windows
- `pnpm build:linux` - Build for Linux
- `pnpm build:all` - Build for all platforms
- `pnpm create-dmg` - Create DMG file for macOS (after building)
- `pnpm lint` - Run ESLint

### Adding New Features

1. **Frontend**: Add React components in `src/components/`
2. **Backend**: Add Rust commands in `src-tauri/src/lib.rs`
3. **API**: Update TypeScript interfaces as needed
4. **i18n**: Add translations in `src/lib/i18n.ts`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/moonraker-host-scanner/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/moonraker-host-scanner/discussions)

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop application framework
- [Next.js](https://nextjs.org/) - React framework
- [Moonraker](https://moonraker.readthedocs.io/) - 3D printer API
- [shadcn/ui](https://ui.shadcn.com/) - UI components
