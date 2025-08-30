#!/usr/bin/env node

/**
 * Скрипт для создания релиза с автоматическим обновлением версии и git tag
 * Использование: node scripts/release.js <version> [message]
 * Пример: node scripts/release.js 0.1.1 "Новые функции"
 */

const { execSync } = require('child_process');
const { updateVersion, validateVersion } = require('./update-version.js');

function runCommand(command, description) {
  try {
    console.log(`🔄 ${description}...`);
    const result = execSync(command, { encoding: 'utf8', stdio: 'pipe' });
    console.log(`✅ ${description} завершено`);
    return result;
  } catch (error) {
    console.error(`❌ Ошибка при ${description}:`, error.message);
    throw error;
  }
}

function checkGitStatus() {
  try {
    const status = execSync('git status --porcelain', { encoding: 'utf8' });
    if (status.trim()) {
      console.error('❌ Есть незакоммиченные изменения. Сначала закоммитьте их:');
      console.error(status);
      process.exit(1);
    }
  } catch (error) {
    console.error('❌ Ошибка при проверке git статуса:', error.message);
    process.exit(1);
  }
}

function checkGitTag(tag) {
  try {
    execSync(`git rev-parse v${tag}`, { stdio: 'ignore' });
    console.error(`❌ Тег v${tag} уже существует`);
    process.exit(1);
  } catch (error) {
    // Тег не существует, это хорошо
  }
}

function createRelease(version, message = '') {
  const tag = `v${version}`;
  const releaseMessage = message || `Release ${tag}`;

  console.log(`🚀 Создание релиза ${tag}...`);
  console.log(`📝 Сообщение: ${releaseMessage}`);
  console.log('');

  // Проверяем git статус
  checkGitStatus();

  // Проверяем, не существует ли уже такой тег
  checkGitTag(version);

  // Обновляем версии во всех файлах
  updateVersion(version);

  // Проверяем, что версия действительно обновилась
  try {
    const packageVersion = execSync('node -p "require(\'./package.json\').version"', { encoding: 'utf8' }).trim();
    if (packageVersion !== version) {
      console.error(`❌ Ошибка: версия в package.json (${packageVersion}) не совпадает с запрошенной (${version})`);
      process.exit(1);
    }
    console.log(`✅ Версия успешно обновлена до ${version}`);
  } catch (error) {
    console.error('❌ Ошибка при проверке версии:', error.message);
    process.exit(1);
  }

  // Коммитим изменения версии
  try {
    runCommand('git add .', 'Добавление изменений в git');
    const commitMessage = `chore: bump version to ${version}

${releaseMessage}`;
    runCommand(`git commit -m "${commitMessage}"`, 'Коммит изменений версии');
  } catch (error) {
    console.error('❌ Ошибка при коммите изменений версии');
    process.exit(1);
  }

  // Создаем git tag
  try {
    runCommand(`git tag -a ${tag} -m "${releaseMessage}"`, 'Создание git тега');
  } catch (error) {
    console.error('❌ Ошибка при создании git тега');
    process.exit(1);
  }

  // Показываем информацию о созданном теге
  try {
    const tagInfo = execSync(`git show ${tag} --stat`, { encoding: 'utf8' });
    console.log('\n📋 Информация о созданном теге:');
    console.log(tagInfo);
  } catch (error) {
    console.log('⚠️  Не удалось показать информацию о теге');
  }

  console.log('\n🎉 Релиз успешно создан!');
  console.log(`📦 Тег: ${tag}`);
  console.log(`📝 Сообщение: ${releaseMessage}`);
  console.log('');
  console.log('📤 Для публикации выполните:');
  console.log(`   git push origin ${tag}`);
  console.log('   git push origin main');
}

function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0) {
    console.log('📖 Использование: node scripts/release.js <version> [message]');
    console.log('📖 Пример: node scripts/release.js 0.1.1 "Новые функции"');
    console.log('📖 Пример: node scripts/release.js 0.1.1');
    process.exit(1);
  }

  const version = args[0];
  const message = args[1] || '';

  validateVersion(version);
  createRelease(version, message);
}

if (require.main === module) {
  main();
}

module.exports = { createRelease };
