# Translations Structure

This directory contains all translation files for the Moonraker Host Scanner application.

## File Structure

```
translations/
├── index.ts      # Main export file with interface and utility functions
├── en.ts         # English translations
├── ru.ts         # Russian translations
├── de.ts         # German translations
└── README.md     # This documentation file
```

## Adding a New Language

To add a new language (e.g., German):

1. Create a new file `de.ts` in the `translations/` directory
2. Export a constant with all required translation keys:

```typescript
export const de = {
  // Header
  networkScanner: "Moonraker Host Scanner",
  discoverHosts: "Entdecke Klipper 3D-Drucker in deinem lokalen Netzwerk",
  settings: "Einstellungen",
  
  // ... all other translation keys
}
```

3. Import and add the new language to the `translations` object in `index.ts`:

```typescript
import { de } from './de'

export const translations: Record<string, Translations> = {
  en,
  ru,
  de, // German language
}
```

## Translation Keys

All translation keys are defined in the `Translations` interface in `index.ts`. Each key corresponds to a specific UI element or message in the application.

### Categories

- **Header**: Main application header and navigation
- **Stats Cards**: Statistics display cards
- **Scan Controls**: Network scanning interface elements
- **Table Headers**: Data table column headers
- **Expanded Row**: Printer control buttons and actions
- **Settings Dialog**: Application settings interface
- **Network Tab**: Network configuration settings
- **SSH Tab**: SSH connection settings
- **Notifications Tab**: Notification preferences
- **About Tab**: Application information
- **Language Tab**: Language selection interface
- **Table Content**: Dynamic content in data tables
- **Hostname Management**: Hostname editing functionality
- **Auto-refresh**: Automatic refresh indicators

## Supported Languages

The application currently supports the following languages:

- **🇺🇸 English** (`en`) - Default language
- **🇷🇺 Russian** (`ru`) - Полный перевод
- **🇩🇪 German** (`de`) - Vollständige Übersetzung

## Usage

Import and use translations in your components:

```typescript
import { useTranslation } from '@/lib/i18n'

function MyComponent() {
  const t = useTranslation('en') // or 'ru', 'de', etc.
  
  return <div>{t.networkScanner}</div>
}
```

## Best Practices

1. **Keep keys organized**: Group related translations together with comments
2. **Use descriptive names**: Make translation keys self-explanatory
3. **Maintain consistency**: Use the same key names across all language files
4. **Test thoroughly**: Verify all translations display correctly in the UI
5. **Update interface**: When adding new keys, update the `Translations` interface

## Maintenance

- When adding new UI elements, add corresponding translation keys
- When modifying existing UI, update affected translation keys
- Regularly review and improve translation quality
- Consider using translation management tools for larger projects
