# Release Troubleshooting Guide

## Проблема: "Pattern does not match any files"

### Описание проблемы
При создании GitHub Release получается ошибка:
```
🤔 Pattern 'artifacts/macos-arm/**/*.dmg' does not match any files.
🤔 Pattern 'artifacts/macos-arm/**/*.app' does not match any files.
🤔 Pattern 'artifacts/macos-arm/**/*.tar.gz' does not match any files.
```

### Причины проблемы

1. **Неправильная структура артефактов**
   - Файлы не копируются в правильную директорию
   - Неправильные пути в workflow

2. **Проблемы со сборкой**
   - Tauri build не создает ожидаемые файлы
   - Отсутствуют иконки или ресурсы

3. **Проблемы с DMG созданием**
   - `create-dmg` не установлен или не работает
   - Отсутствует `icon.icns`

### Решения

#### 1. Проверка структуры артефактов

Запустите скрипт проверки:
```bash
./scripts/check-artifacts.sh
```

#### 2. Исправление workflow

Основные изменения в `.github/workflows/main.yml`:

1. **Упрощение путей файлов**:
   ```yaml
   files: |
     artifacts/macos-arm/*.dmg
     artifacts/macos-arm/*.tar.gz
   ```

2. **Создание tar.gz как fallback**:
   ```bash
   # Если нет DMG, создаем tar.gz
   if [ $DMG_COUNT -eq 0 ] && [ $APP_COUNT -gt 0 ]; then
     tar -czf "artifacts/macos-arm/$APP_NAME.tar.gz" -C artifacts/macos-arm "$APP_NAME.app"
   fi
   ```

3. **Дополнительная проверка в release job**:
   ```bash
   # Создаем tar.gz если нет файлов для релиза
   if [ $(find ./artifacts -name "*.dmg" | wc -l) -eq 0 ] && [ $(find ./artifacts -name "*.tar.gz" | wc -l) -eq 0 ]; then
     # Создаем tar.gz из .app
   fi
   ```

#### 3. Проверка иконок

Убедитесь, что файл `src-tauri/icons/icon.icns` существует:
```bash
ls -la src-tauri/icons/
```

#### 4. Тестирование workflow

Используйте тестовый workflow `.github/workflows/test-release.yml`:
1. Перейдите в GitHub Actions
2. Выберите "Test Release Process"
3. Нажмите "Run workflow"
4. Введите тег (например, v0.0.2)

### Отладка

#### Логи для проверки

1. **Build logs**:
   ```
   === Build output verification ===
   Target directory contents:
   Bundle directory contents:
   ```

2. **Artifacts logs**:
   ```
   === Artifacts structure ===
   === File counts ===
   ```

3. **Release logs**:
   ```
   === Downloaded artifacts verification ===
   === Final artifacts check ===
   ```

#### Команды для локальной проверки

```bash
# Проверка сборки
pnpm tauri build --target aarch64-apple-darwin

# Проверка структуры
find src-tauri/target/aarch64-apple-darwin/release -type f

# Проверка артефактов
./scripts/check-artifacts.sh
```

### Частые проблемы

#### 1. "No package.json found"
```bash
# Решение: перейти в правильную директорию
cd MoonrakerHostScanner
pnpm install
```

#### 2. "icon.icns not found"
```bash
# Решение: создать иконку
# Используйте любой .icns файл или конвертируйте PNG
cp path/to/icon.icns src-tauri/icons/
```

#### 3. "create-dmg not found"
```bash
# Решение: установить create-dmg
brew install create-dmg
```

#### 4. "Permission denied"
```bash
# Решение: сделать скрипты исполняемыми
chmod +x scripts/*.sh
```

### Успешный релиз

После исправлений релиз должен содержать:
- ✅ `.dmg` файл (предпочтительно)
- ✅ `.tar.gz` файл (fallback)
- ✅ Правильное описание
- ✅ Корректные инструкции по установке

### Контакты

Если проблемы остаются, проверьте:
1. GitHub Actions logs полностью
2. Структуру артефактов
3. Права доступа к репозиторию
4. Настройки GitHub Secrets
