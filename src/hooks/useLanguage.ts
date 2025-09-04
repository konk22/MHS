import { useState, useEffect } from 'react'

type Language = "en" | "ru"

export function useLanguage() {
  const [language, setLanguage] = useState<Language>("en")
  const [isLoaded, setIsLoaded] = useState(false)

  // Load language from localStorage
  useEffect(() => {
    const savedLanguage = localStorage.getItem("networkScanner_language")
    if (savedLanguage && ["en", "ru"].includes(savedLanguage)) {
      setLanguage(savedLanguage as Language)
    }
    setIsLoaded(true)
  }, [])

  // Save language to localStorage
  useEffect(() => {
    if (isLoaded) {
      localStorage.setItem("networkScanner_language", language)
    }
  }, [language, isLoaded])

  return {
    language,
    setLanguage,
    isLoaded
  }
}
