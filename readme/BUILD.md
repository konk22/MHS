# 🏗️ BUILD.md - Руководство по сборке MoonrakerHostScanner

## 📋 Обзор

Этот документ содержит подробные инструкции по сборке и развертыванию MoonrakerHostScanner - современного десктопного приложения для управления 3D-принтерами с Moonraker.

## 🎯 Архитектура проекта

### Frontend (React + TypeScript)
- **Next.js 15** - React фреймворк
- **React 18** - Современные хуки и функциональные компоненты
- **TypeScript** - Типобезопасность
- **Tailwind CSS** - Утилитарный CSS фреймворк
- **Shadcn/ui** - Компоненты UI

### Backend (Rust + Tauri)
- **Tauri 2.0** - Кроссплатформенный десктопный фреймворк
- **Rust** - Высокопроизводительный системный язык
- **Tokio** - Асинхронная среда выполнения
- **Reqwest** - HTTP клиент

## 🛠️ Требования к системе

### Общие требования
- **Node.js** 18.0.0 или выше
- **pnpm** 8.0.0 или выше (рекомендуется)
- **Git** для клонирования репозитория

### Rust требования
- **Rust** 1.70.0 или выше
- **Cargo** (включается с Rust)
- **rustup** для управления версиями Rust

### Платформо-специфичные требования

#### macOS
```bash
# Установка Xcode Command Line Tools
xcode-select --install

# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Установка Node.js (через Homebrew)
brew install node

# Установка pnpm
npm install -g pnpm
```

#### Windows
```bash
# Установка Rust
winget install Rust.Rust
# или скачать с https://rustup.rs/

# Установка Node.js
winget install OpenJS.NodeJS
# или скачать с https://nodejs.org/

# Установка pnpm
npm install -g pnpm
```

#### Linux (Ubuntu/Debian)
```bash
# Обновление системы
sudo apt update && sudo apt upgrade

# Установка зависимостей
sudo apt install curl build-essential libssl-dev pkg-config

# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Установка Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Установка pnpm
npm install -g pnpm
```

## 🚀 Быстрая сборка

### 1. Клонирование репозитория
```bash
git clone https://github.com/your-username/MoonrakerHostScanner.git
cd MoonrakerHostScanner
```

### 2. Установка зависимостей
```bash
# Установка Node.js зависимостей
pnpm install

# Проверка Rust зависимостей (автоматически при первой сборке)
cargo check
```

### 3. Разработка
```bash
# Запуск в режиме разработки
pnpm tauri:dev
```

### 4. Продакшен сборка
```bash
# Сборка для текущей платформы
pnpm tauri:build
```

## 🔧 Детальная сборка

### Структура проекта
```
MoonrakerHostScanner/
├── src/                    # Frontend исходный код
│   ├── app/               # Next.js app directory
│   ├── components/        # React компоненты
│   ├── hooks/            # Пользовательские хуки
│   ├── lib/              # Утилиты и библиотеки
│   └── styles/           # CSS стили
├── src-tauri/            # Rust backend
│   ├── src/              # Rust исходный код
│   ├── Cargo.toml        # Rust зависимости
│   └── tauri.conf.json   # Tauri конфигурация
├── public/               # Статические ресурсы
├── docs/                 # Документация
└── scripts/              # Скрипты сборки
```

### Конфигурация Tauri

#### tauri.conf.json
```json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devPath": "http://localhost:3000",
    "distDir": "../out"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.moonrakerhostscanner.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Moonraker Host Scanner",
        "width": 1200,
        "height": 800
      }
    ]
  }
}
```

### Оптимизация сборки

#### Frontend оптимизации
```typescript
// next.config.mjs
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true
  },
  experimental: {
    optimizeCss: true,
    optimizePackageImports: ['lucide-react']
  }
}

export default nextConfig
```

#### Rust оптимизации
```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = 'abort'
strip = true

[profile.dev]
opt-level = 0
debug = true
```

## 📦 Сборка для разных платформ

### macOS
```bash
# Сборка для macOS
pnpm tauri:build --target x86_64-apple-darwin

# Создание DMG
pnpm tauri:build --target x86_64-apple-darwin --config tauri.conf.json
```

### Windows
```bash
# Сборка для Windows
pnpm tauri:build --target x86_64-pc-windows-msvc

# Создание MSI
pnpm tauri:build --target x86_64-pc-windows-msvc --config tauri.conf.json
```

### Linux
```bash
# Сборка для Linux
pnpm tauri:build --target x86_64-unknown-linux-gnu

# Создание AppImage
pnpm tauri:build --target x86_64-unknown-linux-gnu --config tauri.conf.json
```

## 🔍 Отладка и диагностика

### Логи разработки
```bash
# Включение подробных логов
RUST_LOG=debug pnpm tauri:dev

# Логи Tauri
pnpm tauri:dev --log-level debug
```

### Проверка зависимостей
```bash
# Проверка Node.js зависимостей
pnpm audit

# Проверка Rust зависимостей
cargo audit

# Обновление зависимостей
pnpm update
cargo update
```

### Анализ размера бандла
```bash
# Анализ размера Next.js бандла
pnpm build
npx @next/bundle-analyzer

# Анализ размера Rust биндаря
cargo install cargo-bloat
cargo bloat --release
```

## 🚀 Производительность

### Оптимизации сборки
- **Tree shaking** для удаления неиспользуемого кода
- **Code splitting** для разделения бандлов
- **Lazy loading** для компонентов
- **Memoization** для дорогих вычислений

### Мониторинг производительности
```bash
# Профилирование React
pnpm tauri:dev --profile

# Профилирование Rust
cargo install flamegraph
cargo flamegraph
```

## 🔒 Безопасность

### Проверки безопасности
```bash
# Проверка уязвимостей Node.js
pnpm audit

# Проверка уязвимостей Rust
cargo audit

# Проверка лицензий
pnpm license-checker
cargo license
```

### Конфигурация безопасности
```json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
    }
  }
}
```

## 📊 Метрики сборки

### Время сборки
- **Development**: ~30 секунд
- **Production**: ~2-3 минуты
- **Full release**: ~5-10 минут

### Размер бандла
- **Frontend**: ~2-3 MB
- **Backend**: ~5-8 MB
- **Total**: ~10-15 MB

### Оптимизации
- **Gzip compression**: ~60% уменьшение
- **Brotli compression**: ~70% уменьшение
- **Code splitting**: ~40% уменьшение начальной загрузки

## 🐛 Устранение неполадок

### Частые проблемы

#### Ошибки Rust
```bash
# Очистка кэша Rust
cargo clean
cargo update

# Проверка версии Rust
rustc --version
cargo --version
```

#### Ошибки Node.js
```bash
# Очистка кэша Node.js
rm -rf node_modules
rm -rf .next
pnpm install

# Проверка версии Node.js
node --version
pnpm --version
```

#### Ошибки Tauri
```bash
# Очистка кэша Tauri
rm -rf src-tauri/target
cargo clean

# Переустановка Tauri CLI
cargo install tauri-cli --force
```

### Логи ошибок
```bash
# Подробные логи
RUST_LOG=trace pnpm tauri:dev

# Логи в файл
pnpm tauri:dev 2>&1 | tee build.log
```

## 📈 CI/CD

### GitHub Actions
```yaml
name: Build and Release

on:
  push:
    tags: ['v*']

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '18'
      - uses: actions/setup-rust@v1
        with:
          rust-version: '1.70'
      
      - name: Install pnpm
        run: npm install -g pnpm
      
      - name: Install dependencies
        run: pnpm install
      
      - name: Build application
        run: pnpm tauri:build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: moonraker-host-scanner-${{ matrix.os }}
          path: src-tauri/target/release/
```

## 📚 Дополнительные ресурсы

### Документация
- [Tauri Documentation](https://tauri.app/docs)
- [Next.js Documentation](https://nextjs.org/docs)
- [Rust Documentation](https://doc.rust-lang.org)

### Сообщество
- [Tauri Discord](https://discord.gg/tauri)
- [Rust Community](https://www.rust-lang.org/community)
- [Next.js Community](https://nextjs.org/community)

