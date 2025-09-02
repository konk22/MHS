import { useState, useEffect, useCallback } from 'react';

export interface UpdateInfo {
  update_available: boolean;
  current_version: string;
  latest_version?: string;
  latest_release?: {
    tag_name: string;
    name: string;
    body?: string;
    created_at: string;
    published_at?: string;
    assets: Array<{
      name: string;
      browser_download_url: string;
      size: number;
      content_type: string;
    }>;
  };
  error?: string;
  last_check: string;
}

export interface RepositoryInfo {
  repository_url: string;
  releases_url: string;
}

export const useUpdater = () => {
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [repositoryInfo, setRepositoryInfo] = useState<RepositoryInfo>({
    repository_url: 'https://github.com/konk22/MHS',
    releases_url: 'https://github.com/konk22/MHS/releases',
  });
  const [isChecking, setIsChecking] = useState(false);
  const [lastCheck, setLastCheck] = useState<Date | null>(null);

  // Tauri API function
  const invokeTauri = async (command: string, args?: any) => {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      return await (window as any).__TAURI__.core.invoke(command, args)
    }
    throw new Error('Tauri API not available')
  }

  // Initialize repository info
  useEffect(() => {
    const initRepositoryInfo = async () => {
      try {
        const [repoUrl, releasesUrl] = await Promise.all([
          invokeTauri('get_repository_url_command'),
          invokeTauri('get_releases_url_command'),
        ]);

        setRepositoryInfo({
          repository_url: repoUrl,
          releases_url: releasesUrl,
        });
              } catch (error) {
          console.error('Failed to get repository info:', error);
          // Keep default URLs if API fails
        }
    };

    initRepositoryInfo();
  }, []);

  // Check for updates
  const checkForUpdates = useCallback(async () => {
    setIsChecking(true);
    try {
      const result = await invokeTauri('check_for_updates_command');
      setUpdateInfo(result);
      setLastCheck(new Date());
    } catch (error) {
      console.error('Failed to check for updates:', error);
      setUpdateInfo({
        update_available: false,
        current_version: '0.0.0',
        error: error as string,
        last_check: new Date().toISOString(),
      });
    } finally {
      setIsChecking(false);
    }
  }, []);

  // Auto-check for updates every hour
  useEffect(() => {
    // Initial check
    checkForUpdates();

    // Set up interval for hourly checks
    const interval = setInterval(checkForUpdates, 60 * 60 * 1000); // 1 hour

    return () => clearInterval(interval);
  }, [checkForUpdates]);

  // Open repository in browser
  const openRepository = useCallback(async () => {
    const url = repositoryInfo?.repository_url || 'https://github.com/konk22/MHS';
    console.log('Opening repository URL:', url);
    try {
      await invokeTauri('open_url_in_browser_command', { url });
    } catch (error) {
      console.error('Failed to open repository URL:', error);
      // Fallback to window.open
      const newWindow = window.open(url, '_blank');
      if (!newWindow) {
        console.error('Both Tauri and window.open failed');
      }
    }
  }, [repositoryInfo, invokeTauri]);

  // Open releases in browser
  const openReleases = useCallback(async () => {
    const url = repositoryInfo?.releases_url || 'https://github.com/konk22/MHS/releases';
    console.log('Opening releases URL:', url);
    try {
      await invokeTauri('open_url_in_browser_command', { url });
    } catch (error) {
      console.error('Failed to open releases URL:', error);
      // Fallback to window.open
      const newWindow = window.open(url, '_blank');
      if (!newWindow) {
        console.error('Both Tauri and window.open failed');
      }
    }
  }, [repositoryInfo, invokeTauri]);

  return {
    updateInfo,
    repositoryInfo,
    isChecking,
    lastCheck,
    checkForUpdates,
    openRepository,
    openReleases,
  };
};
