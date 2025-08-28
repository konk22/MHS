# Contributing to Moonraker Host Scanner

🤝 **Thank you for your interest in contributing to Moonraker Host Scanner!**

This document provides guidelines and information for contributors to help make the contribution process smooth and effective.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Feature Requests](#feature-requests)
- [Documentation](#documentation)

## 🤝 Code of Conduct

### Our Standards

We are committed to providing a welcoming and inspiring community for all. By participating in this project, you agree to:

- **Be respectful** and inclusive of all contributors
- **Be collaborative** and open to different viewpoints
- **Be constructive** in feedback and criticism
- **Be professional** in all interactions

### Enforcement

Unacceptable behavior will not be tolerated. Please report any violations to the project maintainers.

## 🚀 Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Node.js** 18.0.0 or higher
- **pnpm** 8.0.0 or higher
- **Rust** 1.70.0 or higher
- **Git** for version control
- **Basic knowledge** of React, TypeScript, and Rust

### Fork and Clone

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/MoonrakerHostScanner.git
   cd MoonrakerHostScanner
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/MoonrakerHostScanner.git
   ```

## 🛠️ Development Setup

### 1. Install Dependencies

```bash
# Install Node.js dependencies
pnpm install

# Verify Rust toolchain
cargo --version
rustc --version
```

### 2. Development Environment

```bash
# Start development server
pnpm tauri:dev

# Build for testing
pnpm tauri:build
```

### 3. Code Quality Tools

```bash
# Run linting
pnpm lint

# Run type checking
pnpm type-check

# Run tests (if available)
pnpm test
```

## 📝 Code Style Guidelines

### TypeScript/React Guidelines

#### File Organization
```
src/
├── app/                    # Next.js app directory
├── components/            # React components
│   ├── ui/               # Reusable UI components
│   └── feature/          # Feature-specific components
├── lib/                  # Utilities and helpers
├── hooks/                # Custom React hooks
└── types/                # TypeScript type definitions
```

#### Component Structure
```typescript
// Component naming: PascalCase
export function NetworkScanner() {
  // Hooks first
  const [state, setState] = useState()
  
  // Event handlers
  const handleClick = () => {
    // Implementation
  }
  
  // Render
  return (
    <div>
      {/* JSX */}
    </div>
  )
}
```

#### TypeScript Best Practices
```typescript
// Use interfaces for object shapes
interface HostInfo {
  id: string
  hostname: string
  ip_address: string
  status: 'online' | 'offline'
}

// Use type aliases for unions
type PrinterStatus = 'printing' | 'paused' | 'error' | 'standby'

// Use const assertions for constants
const STATUS_CONFIG = {
  printing: { color: 'green', text: 'Printing' }
} as const
```

### Rust Guidelines

#### File Organization
```
src-tauri/src/
├── lib.rs                # Main library file
├── commands/             # Tauri commands
├── models/               # Data structures
├── network/              # Network utilities
└── utils/                # Helper functions
```

#### Code Style
```rust
// Use snake_case for functions and variables
pub async fn scan_network(subnet: &str) -> Result<Vec<HostInfo>, Error> {
    // Implementation
}

// Use PascalCase for types
#[derive(Debug, Serialize, Deserialize)]
pub struct HostInfo {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
}

// Use SCREAMING_SNAKE_CASE for constants
const DEFAULT_PORT: u16 = 7125;
const SCAN_TIMEOUT: Duration = Duration::from_secs(5);
```

#### Error Handling
```rust
// Use Result for fallible operations
pub async fn get_printer_info(host: &str) -> Result<PrinterInfo, MoonrakerError> {
    let response = reqwest::get(&format!("http://{}:7125/server/info", host))
        .await
        .map_err(|e| MoonrakerError::NetworkError(e.to_string()))?;
    
    response.json::<PrinterInfo>()
        .await
        .map_err(|e| MoonrakerError::ParseError(e.to_string()))
}
```

### Internationalization

#### Adding New Translations

1. **Add translation key** to all language files:
   ```typescript
   // src/lib/translations/en.ts
   export const en: Translations = {
     // ... existing translations
     newFeature: "New Feature"
   }
   
   // src/lib/translations/ru.ts
   export const ru: Translations = {
     // ... existing translations
     newFeature: "Новая функция"
   }
   
   // src/lib/translations/de.ts
   export const de: Translations = {
     // ... existing translations
     newFeature: "Neue Funktion"
   }
   ```

2. **Update interface** in `src/lib/translations/index.ts`:
   ```typescript
   export interface Translations {
     // ... existing fields
     newFeature: string
   }
   ```

3. **Use in components**:
   ```typescript
   const { t } = useTranslation()
   return <div>{t.newFeature}</div>
   ```

## 🧪 Testing Guidelines

### Frontend Testing

```typescript
// Component testing with React Testing Library
import { render, screen } from '@testing-library/react'
import { NetworkScanner } from './network-scanner'

test('displays scan button', () => {
  render(<NetworkScanner />)
  expect(screen.getByText('Scan Network')).toBeInTheDocument()
})
```

### Backend Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_scan_network() {
        let result = scan_network("192.168.1.0/24").await;
        assert!(result.is_ok());
    }
}
```

### Integration Testing

```bash
# Test complete application flow
pnpm tauri:dev
# Manually test all features
```

## 🔄 Pull Request Process

### 1. Create Feature Branch

```bash
# Create and switch to feature branch
git checkout -b feature/amazing-feature

# Or for bug fixes
git checkout -b fix/bug-description
```

### 2. Make Changes

- **Write clear, focused commits**
- **Follow code style guidelines**
- **Add tests for new features**
- **Update documentation as needed**

### 3. Commit Messages

Use conventional commit format:
```
type(scope): description

feat(network): add subnet configuration
fix(ui): resolve webcam modal positioning
docs(readme): update installation instructions
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

### 4. Push and Create PR

```bash
# Push to your fork
git push origin feature/amazing-feature

# Create Pull Request on GitHub
```

### 5. PR Template

Use the provided PR template and include:

- **Description** of changes
- **Screenshots** (if UI changes)
- **Testing** performed
- **Breaking changes** (if any)
- **Related issues** (if any)

### 6. Review Process

- **Code review** by maintainers
- **Automated checks** must pass
- **Manual testing** may be required
- **Address feedback** promptly

## 🐛 Issue Reporting

### Before Reporting

1. **Search existing issues** for duplicates
2. **Check documentation** for solutions
3. **Test with latest version**

### Issue Template

```markdown
## Bug Description
Brief description of the issue

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., macOS 14.0]
- Node.js: [e.g., 18.17.0]
- Rust: [e.g., 1.70.0]
- Application Version: [e.g., 0.1.0]

## Additional Information
Screenshots, logs, or other relevant information
```

## 💡 Feature Requests

### Before Requesting

1. **Check existing features** to avoid duplicates
2. **Consider implementation complexity**
3. **Ensure alignment with project goals**

### Feature Request Template

```markdown
## Feature Description
Clear description of the requested feature

## Use Case
Why this feature is needed

## Proposed Implementation
How you think it should work

## Alternatives Considered
Other approaches you've considered

## Additional Context
Any other relevant information
```

## 📚 Documentation

### Code Documentation

#### TypeScript/React
```typescript
/**
 * Scans the network for Moonraker hosts
 * @param subnets - Array of subnets to scan
 * @returns Promise resolving to array of discovered hosts
 */
export async function scanNetwork(subnets: string[]): Promise<HostInfo[]> {
  // Implementation
}
```

#### Rust
```rust
/// Scans the network for Moonraker hosts
/// 
/// # Arguments
/// * `subnet` - Subnet to scan (e.g., "192.168.1.0/24")
/// 
/// # Returns
/// * `Result<Vec<HostInfo>, MoonrakerError>` - Discovered hosts or error
pub async fn scan_network(subnet: &str) -> Result<Vec<HostInfo>, MoonrakerError> {
    // Implementation
}
```

### User Documentation

- **README.md**: Project overview and quick start
- **BUILD.md**: Build and deployment instructions
- **API documentation**: For developers integrating with the app

## 🏷️ Version Management

### Semantic Versioning

- **Major** (1.0.0): Breaking changes
- **Minor** (1.1.0): New features, backward compatible
- **Patch** (1.1.1): Bug fixes, backward compatible

### Release Process

1. **Update version** in `package.json` and `Cargo.toml`
2. **Update changelog** with new features/fixes
3. **Create release tag**: `git tag v1.0.0`
4. **Push tag**: `git push origin v1.0.0`
5. **Create GitHub release** with build artifacts

## 🤝 Community Guidelines

### Communication

- **Be respectful** in all interactions
- **Use clear language** and provide context
- **Ask questions** when unsure
- **Help others** when possible

### Recognition

Contributors will be recognized in:
- **README.md** contributors section
- **Release notes** for significant contributions
- **GitHub contributors** page

## 📞 Getting Help

### Resources

- **Documentation**: README.md, BUILD.md
- **Issues**: GitHub Issues for bug reports
- **Discussions**: GitHub Discussions for questions
- **Code**: Source code and comments

### Contact

- **Maintainers**: @project-maintainers
- **Community**: GitHub Discussions
- **Security**: Private security issues

---

**Thank you for contributing to Moonraker Host Scanner! 🎉**

Your contributions help make this project better for everyone in the 3D printing community.
