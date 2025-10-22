/**
 * DisableContextMenu Component
 * 
 * This component disables the right-click context menu globally,
 * except for input fields, textareas, and contenteditable elements.
 * For input fields, it shows a custom context menu with only copy, cut, paste operations.
 * It also prevents text selection on non-input elements.
 */

"use client"

import { useEffect } from 'react'
import { useTranslation } from '@/lib/i18n'

export function DisableContextMenu() {
  useEffect(() => {
    // Get current language from localStorage or default to English
    const getCurrentLanguage = () => {
      if (typeof window !== 'undefined') {
        const settings = localStorage.getItem('networkScanner_settings')
        if (settings) {
          try {
            const parsed = JSON.parse(settings)
            return parsed.language || 'en'
          } catch {
            return 'en'
          }
        }
      }
      return 'en'
    }

    // Get current theme from localStorage or default to light
    const getCurrentTheme = () => {
      if (typeof window !== 'undefined') {
        const settings = localStorage.getItem('networkScanner_settings')
        if (settings) {
          try {
            const parsed = JSON.parse(settings)
            return parsed.theme || 'light'
          } catch {
            return 'light'
          }
        }
      }
      return 'light'
    }

    // Get translations based on current language
    const getTranslations = (language: string) => {
      const translations = {
        en: { copy: 'Copy', cut: 'Cut', paste: 'Paste' },
        ru: { copy: 'Копировать', cut: 'Вырезать', paste: 'Вставить' },
        de: { copy: 'Kopieren', cut: 'Ausschneiden', paste: 'Einfügen' }
      }
      return translations[language as keyof typeof translations] || translations.en
    }

    // Get theme styles based on current theme
    const getThemeStyles = (theme: string) => {
      return theme === 'dark' ? {
        background: '#1f2937',
        border: '#374151',
        text: '#f9fafb',
        hover: '#374151',
        shadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'
      } : {
        background: '#ffffff',
        border: '#d1d5db',
        text: '#111827',
        hover: '#f3f4f6',
        shadow: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'
      }
    }

    // Create custom context menu element
    const createCustomContextMenu = () => {
      const currentTheme = getCurrentTheme()
      const currentLanguage = getCurrentLanguage()
      const t = getTranslations(currentLanguage)
      const themeStyles = getThemeStyles(currentTheme)
      
      const menu = document.createElement('div')
      menu.id = 'custom-context-menu'
      menu.style.cssText = `
        position: fixed;
        background: ${themeStyles.background};
        border: 1px solid ${themeStyles.border};
        border-radius: 6px;
        box-shadow: ${themeStyles.shadow};
        z-index: 10000;
        display: none;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 14px;
        min-width: 160px;
        color: ${themeStyles.text};
      `
      
      const menuItems = [
        { text: t.copy, action: 'copy' },
        { text: t.cut, action: 'cut' },
        { text: t.paste, action: 'paste' }
      ]
      
      menuItems.forEach(item => {
        const menuItem = document.createElement('div')
        menuItem.textContent = item.text
        menuItem.style.cssText = `
          padding: 10px 16px;
          cursor: pointer;
          border-bottom: 1px solid ${themeStyles.border};
          transition: background-color 0.2s ease;
          color: ${themeStyles.text};
        `
        
        menuItem.addEventListener('mouseenter', () => {
          menuItem.style.backgroundColor = themeStyles.hover
        })
        
        menuItem.addEventListener('mouseleave', () => {
          menuItem.style.backgroundColor = 'transparent'
        })
        
        menuItem.addEventListener('click', () => {
          executeCommand(item.action)
          hideCustomContextMenu()
        })
        
        menu.appendChild(menuItem)
      })
      
      // Remove border from last item
      const lastItem = menu.lastElementChild as HTMLElement
      if (lastItem) {
        lastItem.style.borderBottom = 'none'
      }
      
      document.body.appendChild(menu)
      return menu
    }
    
    let customMenu: HTMLElement | null = null
    
    const executeCommand = (command: string) => {
      try {
        document.execCommand(command, false)
      } catch (error) {
        console.warn(`Failed to execute command: ${command}`, error)
      }
    }
    
    const showCustomContextMenu = (event: MouseEvent, target: HTMLElement) => {
      // Remove existing menu if it exists
      if (customMenu) {
        document.body.removeChild(customMenu)
        customMenu = null
      }
      
      // Create new menu with current settings
      customMenu = createCustomContextMenu()
      
      // Position menu at cursor
      customMenu.style.left = `${event.clientX}px`
      customMenu.style.top = `${event.clientY}px`
      customMenu.style.display = 'block'
      
      // Focus the target element for paste operation
      if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') {
        (target as HTMLInputElement | HTMLTextAreaElement).focus()
      }
    }
    
    const hideCustomContextMenu = () => {
      if (customMenu) {
        customMenu.style.display = 'none'
      }
    }
    
    const handleContextMenu = (event: MouseEvent) => {
      const target = event.target as HTMLElement
      const isInputElement = target.tagName === 'INPUT' || 
                           target.tagName === 'TEXTAREA' || 
                           target.contentEditable === 'true' ||
                           target.closest('input') ||
                           target.closest('textarea') ||
                           target.closest('[contenteditable="true"]')
      
      if (isInputElement) {
        // Show custom context menu for input elements
        event.preventDefault()
        showCustomContextMenu(event, target)
        return false
      } else {
        // Disable context menu for non-input elements
        event.preventDefault()
        hideCustomContextMenu()
        return false
      }
    }

    const handleSelectStart = (event: Event) => {
      // Allow text selection only for input fields, textareas, and contenteditable elements
      const target = event.target as HTMLElement
      const isInputElement = target.tagName === 'INPUT' || 
                           target.tagName === 'TEXTAREA' || 
                           target.contentEditable === 'true' ||
                           target.closest('input') ||
                           target.closest('textarea') ||
                           target.closest('[contenteditable="true"]')
      
      if (!isInputElement) {
        event.preventDefault()
        return false
      }
    }
    
    const handleClick = () => {
      // Hide custom context menu when clicking anywhere
      hideCustomContextMenu()
    }
    
    const handleKeyDown = (event: KeyboardEvent) => {
      // Hide custom context menu on Escape key
      if (event.key === 'Escape') {
        hideCustomContextMenu()
      }
    }

    // Add event listeners
    document.addEventListener('contextmenu', handleContextMenu)
    document.addEventListener('selectstart', handleSelectStart)
    document.addEventListener('click', handleClick)
    document.addEventListener('keydown', handleKeyDown)

    // Cleanup function
    return () => {
      document.removeEventListener('contextmenu', handleContextMenu)
      document.removeEventListener('selectstart', handleSelectStart)
      document.removeEventListener('click', handleClick)
      document.removeEventListener('keydown', handleKeyDown)
      
      // Remove custom menu from DOM
      if (customMenu) {
        document.body.removeChild(customMenu)
      }
    }
  }, [])

  // This component doesn't render anything
  return null
}
