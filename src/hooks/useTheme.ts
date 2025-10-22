import { useState, useEffect, useCallback } from 'react'

type Theme = "light" | "dark" | "system"

export function useTheme() {
  const [theme, setTheme] = useState<Theme>("system")
  const [isLoaded, setIsLoaded] = useState(false)

  // Load theme from localStorage
  useEffect(() => {
    const savedTheme = localStorage.getItem("networkScanner_theme")
    if (savedTheme && ["light", "dark", "system"].includes(savedTheme)) {
      setTheme(savedTheme as Theme)
    }
    setIsLoaded(true)
  }, [])

  // Save theme to localStorage
  useEffect(() => {
    if (isLoaded) {
      localStorage.setItem("networkScanner_theme", theme)
    }
  }, [theme, isLoaded])

  // Apply theme to document
  const applyTheme = useCallback((newTheme: Theme) => {
    const root = document.documentElement

    if (newTheme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"
      root.classList.toggle("dark", systemTheme === "dark")
    } else {
      root.classList.toggle("dark", newTheme === "dark")
    }
  }, [])

  // Apply theme when it changes
  useEffect(() => {
    if (isLoaded) {
      applyTheme(theme)
    }
  }, [theme, isLoaded, applyTheme])

  // Listen for system theme changes
  useEffect(() => {
    if (theme === "system" && isLoaded) {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
      const handleChange = () => applyTheme("system")
      
      mediaQuery.addEventListener("change", handleChange)
      return () => mediaQuery.removeEventListener("change", handleChange)
    }
  }, [theme, isLoaded, applyTheme])

  return {
    theme,
    setTheme,
    isLoaded
  }
}
