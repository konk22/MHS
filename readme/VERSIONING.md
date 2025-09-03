# 📦 VERSIONING.md - Управление версиями MoonrakerHostScanner

## 📋 Обзор

Этот документ описывает систему управления версиями проекта MoonrakerHostScanner и автоматизацию процесса создания релизов.

## 🎯 Файлы с версиями

В проекте версия указана в следующих файлах:

### 1. package.json
```json
{
  "name": "moonraker-host-scanner",
  "version": "0.0.13",
  // ...
}
```

### 2. src-tauri/Cargo.toml
```toml
[package]
name = "moonrakerhostscanner"
version = "0.0.13"
description = "A Tauri App for scanning 3D printer hosts"
```

### 3. src-tauri/tauri.conf.json
```json
{
  "productName": "Moonraker Host Scanner",
  "version": "0.0.13",
  "identifier": "com.tormyhseviv.moonrakerhostscanner"
}
```

### 4. Файлы локализации
```typescript
// src/lib/translations/en.ts
version: "Version 0.0.13"

// src/lib/translations/ru.ts
version: "Версия 0.0.13"

// src/lib/translations/de.ts
version: "Version 0.0.13"
```

## 🚀 Автоматическое управление версиями

### Скрипты

#### 1. Обновление версии
```bash
# Обновить версию во всех файлах
pnpm version:update <version>

# Примеры
pnpm version:update 0.0.14
pnpm version:update 0.1.0
pnpm version:update 1.0.0
```

#### 2. Создание релиза
```bash
# Создать релиз с git tag
pnpm release <version> [message]

# Примеры
pnpm release 0.0.14 "Исправления багов"
pnpm release 0.1.0 "Первая стабильная версия"
pnpm release 0.0.15
```

### Ручное обновление

Если нужно обновить версию вручную:

```bash
# Обновить только версию
node scripts/update-version.js 0.0.14

# Создать релиз с проверками
node scripts/release.js 0.0.14 "Описание изменений"
```

## 📋 Процесс создания релиза

### Автоматический процесс

1. **Проверка git статуса**
   - Убеждается, что нет незакоммиченных изменений
   - Проверяет, не существует ли уже такой тег

2. **Обновление версий**
   - Обновляет версию в `package.json`
   - Обновляет версию в `Cargo.toml`
   - Обновляет версию в `tauri.conf.json`
   - Обновляет версию в файлах локализации

3. **Создание коммита**
   - Добавляет все изменения в git
   - Создает коммит с сообщением `chore: bump version to X.Y.Z`

4. **Создание git тега**
   - Создает аннотированный тег `vX.Y.Z`
   - Добавляет описание релиза

5. **Информация о результате**
   - Показывает информацию о созданном теге
   - Выводит команды для публикации

### Ручной процесс

```bash
# 1. Обновить версии
node scripts/update-version.js 0.1.1

# 2. Проверить изменения
git diff

# 3. Закоммитить изменения
git add .
git commit -m "chore: bump version to 0.1.1"

# 4. Создать тег
git tag -a v0.1.1 -m "Release v0.1.1"

# 5. Опубликовать
git push origin v0.1.1
git push origin main
```

## 📊 Семантическое версионирование

Проект использует [Semantic Versioning](https://semver.org/lang/ru/) (SemVer):

### Формат версии: `MAJOR.MINOR.PATCH`

- **MAJOR** - несовместимые изменения API
- **MINOR** - новая функциональность с обратной совместимостью
- **PATCH** - исправления багов с обратной совместимостью

### Примеры

```bash
# Исправление бага
0.1.0 → 0.1.1

# Новая функция
0.1.1 → 0.2.0

# Критическое изменение
0.2.0 → 1.0.0

# Исправление в стабильной версии
1.0.0 → 1.0.1
```

## 🔧 Настройка

### Предварительные требования

1. **Git репозиторий**
   ```bash
   git init
   git remote add origin <repository-url>
   ```

2. **Права на выполнение скриптов**
   ```bash
   chmod +x scripts/update-version.js
   chmod +x scripts/release.js
   ```

### Проверка текущих версий

```bash
# Проверить версию в package.json
node -p "require('./package.json').version"

# Проверить версию в Cargo.toml
grep '^version =' src-tauri/Cargo.toml

# Проверить версию в tauri.conf.json
node -p "require('./src-tauri/tauri.conf.json').version"
```

## 🚨 Важные моменты

### 1. Синхронизация версий
- Все версии должны быть одинаковыми во всех файлах
- Скрипт автоматически синхронизирует версии
- При ручном изменении проверяйте все файлы

### 2. Git теги
- Теги создаются в формате `vX.Y.Z`
- Каждый тег должен соответствовать коммиту
- Не удаляйте теги после публикации

### 3. Коммиты
- Изменения версии коммитятся автоматически
- Сообщение коммита: `chore: bump version to X.Y.Z`
- Не редактируйте коммиты после создания тега

### 4. Публикация
- Сначала публикуйте тег: `git push origin vX.Y.Z`
- Затем публикуйте ветку: `git push origin main`
- Создавайте GitHub Release для каждого тега

## 📝 Примеры использования

### Создание патча
```bash
# Исправлен баг с уведомлениями
pnpm release 0.1.1 "Исправлен баг с уведомлениями"
```

### Добавление новой функции
```bash
# Добавлена поддержка веб-камер
pnpm release 0.2.0 "Добавлена поддержка веб-камер"
```

### Стабильный релиз
```bash
# Первая стабильная версия
pnpm release 1.0.0 "Первая стабильная версия"
```

### Критическое обновление
```bash
# Исправлена критическая уязвимость
pnpm release 1.0.1 "Исправлена критическая уязвимость безопасности"
```

## 🔍 Отладка

### Частые проблемы

#### 1. "Тег уже существует"
```bash
# Проверить существующие теги
git tag -l

# Удалить тег (если нужно)
git tag -d v0.1.1
git push origin :refs/tags/v0.1.1
```

#### 2. "Есть незакоммиченные изменения"
```bash
# Проверить статус
git status

# Закоммитить изменения
git add .
git commit -m "Описание изменений"
```

#### 3. "Неверный формат версии"
```bash
# Использовать правильный формат
pnpm release 0.1.1  # ✅ Правильно
pnpm release 0.1    # ❌ Неправильно
pnpm release v0.1.1 # ❌ Неправильно
```

## 📚 Дополнительные ресурсы

- [Semantic Versioning](https://semver.org/lang/ru/)
- [Git Tags](https://git-scm.com/book/ru/v2/Git-%D0%B2-%D0%B8%D0%BD%D1%81%D0%B8%D1%81%D1%82%D0%B8%D1%82%D1%83%D1%82%D0%B0%D1%85-%D0%A2%D0%B5%D0%B3%D0%B8)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)

---

**Система версионирования настроена для автоматизации процесса релизов!** 🚀
