# 📦 ARTIFACT_NAMING.md - Именование артефактов сборки

## 📋 Обзор

Артефакты сборки содержат информацию о платформе и архитектуре для удобной идентификации и использования.

## 🏷️ Схема именования

### Формат имени артефакта
```
moonraker-host-scanner-{platform}-{architecture}
```

### Компоненты имени
- **moonraker-host-scanner** - название приложения
- **{platform}** - операционная система (macos, windows)
- **{architecture}** - архитектура процессора (aarch64, x86_64)

## 📊 Таблица артефактов

| Платформа | Архитектура | Имя артефакта | Файл |
|-----------|-------------|---------------|------|
| macOS | ARM64 | `moonraker-host-scanner-macos-aarch64` | `.dmg` |
| macOS | x86_64 | `moonraker-host-scanner-macos-x86_64` | `.dmg` |
| Windows | x86_64 | `moonraker-host-scanner-windows-x86_64` | `.exe` |
| Windows | ARM64 | `moonraker-host-scanner-windows-aarch64` | `.exe` |

## 🎯 Преимущества схемы

### Четкая идентификация
- ✅ **Платформа** - сразу видно для какой ОС
- ✅ **Архитектура** - понятно для какого процессора
- ✅ **Краткие имена** - легко читать и использовать

### Удобство использования
- ✅ **Автоматическая сортировка** по алфавиту
- ✅ **Легкий поиск** нужной версии
- ✅ **Понятные имена** для пользователей

### Совместимость
- ✅ **Универсальные имена** для всех платформ
- ✅ **Масштабируемость** для новых платформ
- ✅ **Простота расширения** для новых архитектур

## 🔧 Структура папок артефактов

### В GitHub Actions
```
artifacts/
├── macos-aarch64/          # macOS ARM64 (Apple Silicon)
├── macos-x86_64/           # macOS x86_64 (Intel)
├── windows-x86_64/         # Windows x86_64 (Intel/AMD)
└── windows-aarch64/        # Windows ARM64 (ARM)
```

### Содержимое папок
Каждая папка содержит:
- **`.dmg`** файлы для macOS
- **`.exe`** файлы для Windows
- **Дополнительные файлы** (установщики, документация)

## 📝 Примеры использования

### Поиск артефакта для конкретной платформы

#### macOS Apple Silicon
```bash
# Имя артефакта
moonraker-host-scanner-macos-aarch64

# Путь к файлам
artifacts/macos-aarch64/
```

#### macOS Intel
```bash
# Имя артефакта
moonraker-host-scanner-macos-x86_64

# Путь к файлам
artifacts/macos-x86_64/
```

#### Windows Intel/AMD
```bash
# Имя артефакта
moonraker-host-scanner-windows-x86_64

# Путь к файлам
artifacts/windows-x86_64/
```

#### Windows ARM
```bash
# Имя артефакта
moonraker-host-scanner-windows-aarch64

# Путь к файлам
artifacts/windows-aarch64/
```

## 🚀 Интеграция с GitHub Actions

### Upload артефактов
```yaml
- name: Upload macOS ARM artifacts
  uses: actions/upload-artifact@v4
  with:
    name: moonraker-host-scanner-macos-aarch64
    path: src-tauri/target/aarch64-apple-darwin/release/bundle/
```

### Download артефактов
```yaml
- name: Download macOS ARM artifacts
  uses: actions/download-artifact@v4
  with:
    name: moonraker-host-scanner-macos-aarch64
    path: ./artifacts/macos-aarch64
```

## 🔍 Отладка

### Проверка имен артефактов
```bash
# В GitHub Actions
echo "Available artifacts:"
ls -la artifacts/

# Проверка содержимого
find artifacts/ -name "*.dmg" -o -name "*.exe"
```

### Поиск по имени
```bash
# Найти все артефакты для macOS
find . -name "*macos*"

# Найти все артефакты для ARM64
find . -name "*aarch64*"
```

## 📋 Чек-лист для релизов

### Перед релизом
- [ ] **Проверить имена артефактов** в workflow
- [ ] **Убедиться в корректности** архитектур
- [ ] **Протестировать сборку** для всех платформ

### После релиза
- [ ] **Проверить артефакты** в GitHub Release
- [ ] **Убедиться в правильности** имен файлов
- [ ] **Проверить доступность** для всех платформ

## 🎯 Лучшие практики

### Именование
1. **Используйте стандартные названия** архитектур
2. **Включайте информацию** о платформе
3. **Следуйте единому формату** для всех артефактов

### Организация
1. **Группируйте по платформе** в папках
2. **Используйте понятные имена** папок
3. **Документируйте структуру** артефактов

### Совместимость
1. **Тестируйте на всех платформах**
2. **Проверяйте совместимость** с архитектурами
3. **Обновляйте документацию** при изменениях

## 🚀 Расширение для новых платформ

### Добавление Linux
```yaml
# Пример для Linux x86_64
name: moonraker-host-scanner-linux-x86_64
path: ./artifacts/linux-x86_64
```

### Добавление других архитектур
```yaml
# Пример для RISC-V
name: moonraker-host-scanner-linux-riscv64
path: ./artifacts/linux-riscv64
```

## 📊 Сравнение с предыдущей схемой

| Аспект | Старая схема | Новая схема |
|--------|-------------|-------------|
| **Длина имени** | Длинные (с target-triple) | Короткие |
| **Читаемость** | Сложная | Простая |
| **Поиск** | Затруднен | Легкий |
| **Сортировка** | Неудобная | Удобная |

---

**Упрощенная схема именования обеспечивает удобство использования!** 🎉

**Теперь имена артефактов краткие и понятные.**
