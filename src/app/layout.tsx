import type { Metadata } from 'next'
import { GeistSans } from 'geist/font/sans'
import { GeistMono } from 'geist/font/mono'
import './globals.css'
import { DisableContextMenu } from '@/components/disable-context-menu'

export const metadata: Metadata = {
  title: 'Moonraker Host Scanner',
  description: 'Network scanner and management tool for Moonraker-enabled 3D printers',
  generator: 'Next.js + Tauri',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <head>
        <style>{`
html {
  font-family: ${GeistSans.style.fontFamily};
  --font-sans: ${GeistSans.variable};
  --font-mono: ${GeistMono.variable};
}
        `}</style>
      </head>
      <body>
        <DisableContextMenu />
        {children}
      </body>
    </html>
  )
}
