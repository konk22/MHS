# Contributing to Moonraker Host Scanner

Thank you for your interest in contributing to Moonraker Host Scanner! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Node.js 18+ and pnpm
- Rust toolchain (latest stable)
- Git

### Development Setup

1. **Fork and clone the repository**:
```bash
git clone https://github.com/yourusername/moonraker-host-scanner.git
cd moonraker-host-scanner
```

2. **Install dependencies**:
```bash
pnpm install
```

3. **Start development server**:
```bash
pnpm tauri:dev
```

## Development Guidelines

### Code Style

- **TypeScript**: Use strict mode, prefer interfaces over types
- **React**: Use functional components with hooks
- **Rust**: Follow Rust conventions, use `cargo fmt` and `cargo clippy`
- **CSS**: Use Tailwind CSS classes, prefer utility-first approach

### Commit Messages

Follow conventional commits format:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

### Pull Request Process

1. **Create a feature branch**:
```bash
git checkout -b feature/your-feature-name
```

2. **Make your changes**:
   - Write clean, documented code
   - Add tests if applicable
   - Update documentation

3. **Test your changes**:
```bash
# Type checking
npx tsc --noEmit --skipLibCheck

# Linting
pnpm lint

# Build test
pnpm build

# Run Tauri in dev mode
pnpm tauri:dev
```

4. **Commit your changes**:
```bash
git add .
git commit -m "feat: add new feature"
```

5. **Push and create PR**:
```bash
git push origin feature/your-feature-name
```

### Pull Request Guidelines

- **Title**: Clear, descriptive title
- **Description**: Explain what and why, not how
- **Screenshots**: Include screenshots for UI changes
- **Tests**: Ensure all tests pass
- **Documentation**: Update docs if needed

## Project Structure

```
MoonrakerHostScanner/
├── src/                    # Next.js frontend
│   ├── app/               # App router pages
│   ├── components/        # React components
│   │   └── ui/           # shadcn/ui components
│   ├── lib/              # Utilities and i18n
│   └── hooks/            # Custom React hooks
├── src-tauri/            # Rust backend
│   ├── src/              # Rust source code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
└── docs/                 # Documentation
```

## Adding New Features

### Frontend (React/TypeScript)

1. **Create component** in `src/components/`
2. **Add translations** in `src/lib/i18n.ts`
3. **Update interfaces** if needed
4. **Add to main page** in `src/app/page.tsx`

### Backend (Rust)

1. **Add command** in `src-tauri/src/lib.rs`
2. **Update handler** in `tauri::generate_handler!`
3. **Add error handling**
4. **Test with frontend**

### Example: Adding a New API Command

**Backend (Rust)**:
```rust
#[tauri::command]
async fn new_command(ip: String) -> Result<String, String> {
    // Implementation
    Ok("Success".to_string())
}

// Add to handler
tauri::generate_handler![
    scan_network,
    control_printer_command,
    open_host_in_browser,
    open_ssh_connection,
    new_command  // Add here
]
```

**Frontend (TypeScript)**:
```typescript
// Add to interface
interface NewCommandResult {
  result: string;
}

// Add to component
const handleNewCommand = async (ip: string) => {
  try {
    const result = await invokeTauri<NewCommandResult>('new_command', { ip });
    console.log(result);
  } catch (error) {
    console.error('Error:', error);
  }
};
```

## Testing

### Manual Testing

1. **Network scanning**: Test with real Moonraker hosts
2. **Printer controls**: Test all control buttons
3. **Webcam streaming**: Test webcam modal and controls
4. **Cross-platform**: Test on different OS if possible

### Automated Testing

```bash
# Type checking
npx tsc --noEmit --skipLibCheck

# Linting
pnpm lint

# Build test
pnpm build
```

## Internationalization

### Adding New Languages

1. **Update `src/lib/i18n.ts`**:
```typescript
export const translations = {
  en: {
    // English translations
  },
  ru: {
    // Russian translations
  },
  es: {  // Add new language
    // Spanish translations
  }
};
```

2. **Add language selector** if needed

### Translation Guidelines

- Use clear, concise language
- Maintain consistent terminology
- Consider cultural differences
- Test with native speakers

## Bug Reports

### Before Submitting

1. **Check existing issues** for duplicates
2. **Test on latest version**
3. **Reproduce the issue**
4. **Gather system information**

### Bug Report Template

```markdown
**Description**
Brief description of the issue

**Steps to Reproduce**
1. Step 1
2. Step 2
3. Step 3

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [e.g., Windows 11, macOS 14, Ubuntu 22.04]
- Version: [e.g., 1.0.0]
- Node.js: [e.g., 18.17.0]
- Rust: [e.g., 1.70.0]

**Additional Information**
Screenshots, logs, etc.
```

## Feature Requests

### Before Submitting

1. **Check existing issues** for similar requests
2. **Consider the scope** and complexity
3. **Think about implementation** details
4. **Consider user impact**

### Feature Request Template

```markdown
**Problem**
Description of the problem this feature would solve

**Proposed Solution**
Description of the proposed solution

**Alternative Solutions**
Other ways to solve the problem

**Additional Context**
Screenshots, mockups, etc.
```

## Code of Conduct

### Our Standards

- **Be respectful** and inclusive
- **Be collaborative** and constructive
- **Be professional** and helpful
- **Be patient** with newcomers

### Enforcement

- Report violations to maintainers
- Maintainers will investigate
- Appropriate action will be taken

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and ideas
- **Documentation**: Check README.md and BUILD.md
- **Tauri Discord**: For Tauri-specific questions

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors page

Thank you for contributing to Moonraker Host Scanner! 🚀
