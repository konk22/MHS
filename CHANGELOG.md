# Changelog

All notable changes to Moonraker Host Scanner will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- System notifications for printer status changes
- German language support
- Enhanced printer status detection using Moonraker API flags
- Image manipulation controls for webcam streams (rotate, flip)

### Changed
- Refactored translation system to use separate language files
- Improved status detection logic with priority-based flag interpretation
- Enhanced error handling and user feedback
- Updated documentation and build instructions

### Fixed
- Auto-refresh functionality after status logic changes
- Hostname persistence during network scans
- Printer status display accuracy
- Webcam modal positioning and controls

## [0.1.0] - 2024-12-19

### Added
- **Network Discovery**: Automatic scanning of local network subnets
- **Host Management**: Discovery and monitoring of Moonraker-enabled printers
- **Printer Control**: Start, pause, cancel, and emergency stop functionality
- **Webcam Integration**: Live streaming with image manipulation controls
- **Quick Access**: Browser and SSH integration for direct host access
- **Settings Persistence**: Save and restore application settings
- **Multi-language Support**: English, Russian, and German translations
- **Real-time Monitoring**: Status updates every second for online hosts
- **Custom Hostnames**: Rename hosts with persistent storage
- **Notification System**: Configurable system notifications for status changes
- **Cross-platform Support**: macOS, Windows, and Linux compatibility

### Technical Features
- **Tauri 2 Integration**: Modern desktop application framework
- **Next.js 15 Frontend**: React-based user interface
- **TypeScript Support**: Full type safety and development experience
- **Rust Backend**: High-performance network operations
- **Moonraker API Integration**: Complete API support for printer control
- **Responsive Design**: Modern UI with dark/light theme support
- **Error Handling**: Comprehensive error management and user feedback

### API Endpoints Supported
- `GET /server/info` - Server information and discovery
- `GET /api/printer` - Printer status and flags
- `POST /printer/print/start` - Start print job
- `POST /printer/print/pause` - Pause print job
- `POST /printer/print/cancel` - Cancel print job
- `POST /printer/emergency_stop` - Emergency stop
- `GET /webcam/?action=stream` - Webcam streaming

### Status Detection
- **Printing**: Active print job
- **Paused**: Print job paused
- **Cancelling**: Print job being cancelled
- **Error**: Printer error state
- **Standby**: Printer ready (mapped from ready flag)
- **Offline**: Host unreachable

---

## Version History

### Development Milestones

#### Phase 1: Core Infrastructure
- ✅ Tauri application setup with Next.js frontend
- ✅ Basic network scanning functionality
- ✅ Moonraker API integration
- ✅ Host discovery and management

#### Phase 2: Printer Control
- ✅ Printer control buttons (start, pause, stop, emergency stop)
- ✅ Real-time status monitoring
- ✅ Webcam streaming integration
- ✅ Browser and SSH access

#### Phase 3: User Experience
- ✅ Multi-language support (English, Russian, German)
- ✅ Settings persistence and customization
- ✅ Custom hostname management
- ✅ System notifications

#### Phase 4: Polish and Optimization
- ✅ Enhanced status detection using API flags
- ✅ Image manipulation for webcam streams
- ✅ Improved error handling
- ✅ Code refactoring and documentation

### Future Roadmap

#### Planned Features
- [ ] Print job history and statistics
- [ ] Temperature monitoring and graphs
- [ ] File management and upload
- [ ] Multiple printer management
- [ ] Advanced notification rules
- [ ] Plugin system for extensions
- [ ] Mobile companion app
- [ ] Cloud synchronization

#### Technical Improvements
- [ ] Performance optimization for large networks
- [ ] Advanced caching strategies
- [ ] Automated testing suite
- [ ] CI/CD pipeline improvements
- [ ] Security enhancements
- [ ] Accessibility improvements

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
