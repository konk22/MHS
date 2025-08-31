#!/usr/bin/env node

/**
 * Скрипт для обновления версии во всех файлах проекта
 * Использование: node scripts/update-version.js <version>
 * Пример: node scripts/update-version.js 0.1.1
 */

const fs = require('fs');
const path = require('path');

function updateVersion(newVersion) {
  console.log(`🔄 Обновление версии на ${newVersion}...`);

  // Файлы для обновления
  const files = [
    {
      path: 'package.json',
      pattern: /"version":\s*"[^"]*"/,
      replacement: `"version": "${newVersion}"`
    },
    {
      path: 'src-tauri/Cargo.toml',
      pattern: /version\s*=\s*"[^"]*"/,
      replacement: `version = "${newVersion}"`
    },
    {
      path: 'src-tauri/tauri.conf.json',
      pattern: /"version":\s*"[^"]*"/,
      replacement: `"version": "${newVersion}"`
    }
  ];

  let updatedFiles = 0;

  files.forEach(file => {
    const filePath = path.join(process.cwd(), file.path);
    
    if (fs.existsSync(filePath)) {
      try {
        let content = fs.readFileSync(filePath, 'utf8');
        const originalContent = content;
        
        content = content.replace(file.pattern, file.replacement);
        
        if (content !== originalContent) {
          fs.writeFileSync(filePath, content, 'utf8');
          console.log(`✅ Обновлен: ${file.path}`);
          updatedFiles++;
        } else {
          // Проверяем, содержит ли файл уже нужную версию
          const currentVersionMatch = content.match(file.pattern);
          if (currentVersionMatch) {
            const currentVersion = currentVersionMatch[0].match(/"[^"]*"/)[0].replace(/"/g, '');
            if (currentVersion === newVersion) {
              console.log(`✅ Уже актуальная версия: ${file.path} (${newVersion})`);
              updatedFiles++;
            } else {
              console.log(`⚠️  Не изменен: ${file.path} (паттерн не найден)`);
            }
          } else {
            console.log(`⚠️  Не изменен: ${file.path} (паттерн не найден)`);
          }
        }
      } catch (error) {
        console.error(`❌ Ошибка при обновлении ${file.path}:`, error.message);
      }
    } else {
      console.error(`❌ Файл не найден: ${file.path}`);
    }
  });

  console.log(`\n📊 Результат: обновлено ${updatedFiles} из ${files.length} файлов`);
  
  if (updatedFiles === files.length) {
    console.log('🎉 Все версии успешно синхронизированы!');
  } else {
    console.log('⚠️  Некоторые файлы не были обновлены. Проверьте ошибки выше.');
  }
}

function validateVersion(version) {
  const versionPattern = /^\d+\.\d+\.\d+$/;
  if (!versionPattern.test(version)) {
    console.error('❌ Неверный формат версии. Используйте формат: X.Y.Z (например, 0.1.0)');
    process.exit(1);
  }
}

function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0) {
    console.log('📖 Использование: node scripts/update-version.js <version>');
    console.log('📖 Пример: node scripts/update-version.js 0.1.1');
    process.exit(1);
  }

  const newVersion = args[0];
  validateVersion(newVersion);
  updateVersion(newVersion);
}

if (require.main === module) {
  main();
}

module.exports = { updateVersion, validateVersion };
