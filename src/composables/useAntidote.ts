import { ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
import { openPath } from '@tauri-apps/plugin-opener';

export interface FileMatch {
  path: string;
  match_count: number;
  format: string;
}

export interface ApplyResult {
  path: string;
  success: boolean;
  error: string | null;
}

export interface ScanProgress {
  scanned: number;
  total: number;
  found: number;
  current: string;
  percent: number;
}

export function useAntidote() {
  const { t } = useI18n();

  const scanPath = ref('');
  const recursive = ref(true);
  const backup = ref(true);
  const scanning = ref(false);
  const applying = ref(false);
  const results = ref<FileMatch[]>([]);
  const selected = ref<Set<string>>(new Set());
  const applyLog = ref<ApplyResult[]>([]);
  const error = ref('');
  const progress = ref<ScanProgress | null>(null);

  const allSelected = computed(
    () =>
      results.value.length > 0 && selected.value.size === results.value.length,
  );

  function toggleAll() {
    selected.value = allSelected.value
      ? new Set()
      : new Set(results.value.map((f) => f.path));
  }

  function toggleFile(path: string) {
    const s = new Set(selected.value);
    if (s.has(path)) s.delete(path);
    else s.add(path);
    selected.value = s;
  }

  async function pickFolder() {
    const folder = await dialogOpen({ directory: true, multiple: false });
    if (folder) scanPath.value = folder as string;
  }

  async function doScan(allDrives = false) {
    error.value = '';
    results.value = [];
    selected.value = new Set();
    applyLog.value = [];
    progress.value = null;
    scanning.value = true;

    const unlisten = await listen<ScanProgress>('scan-progress', (event) => {
      progress.value = event.payload;
    });

    try {
      if (allDrives) {
        results.value = await invoke<FileMatch[]>('scan_all_drives');
      } else {
        if (!scanPath.value) {
          error.value = t('errorNoFolder');
          return;
        }
        results.value = await invoke<FileMatch[]>('scan_directory', {
          path: scanPath.value,
          recursive: recursive.value,
        });
      }
    } catch (e) {
      error.value = String(e);
    } finally {
      unlisten();
      progress.value = null;
      scanning.value = false;
    }
  }

  async function cancelScan() {
    await invoke('cancel_scan');
  }

  async function applyTo(paths: string[]) {
    if (!paths.length) return;
    applying.value = true;
    applyLog.value = [];
    try {
      applyLog.value = await invoke<ApplyResult[]>('apply_replacements', {
        paths,
        backup: backup.value,
      });
      const done = new Set(
        applyLog.value.filter((r) => r.success).map((r) => r.path),
      );
      results.value = results.value.filter((f) => !done.has(f.path));
      selected.value = new Set([...selected.value].filter((p) => !done.has(p)));
    } catch (e) {
      error.value = String(e);
    } finally {
      applying.value = false;
    }
  }

  async function openFile(path: string) {
    try {
      await openPath(path);
    } catch (e) {
      error.value = String(e);
    }
  }

  return {
    scanPath,
    recursive,
    backup,
    scanning,
    applying,
    progress,
    results,
    selected,
    allSelected,
    applyLog,
    error,
    toggleAll,
    toggleFile,
    pickFolder,
    doScan,
    cancelScan,
    applyTo,
    openFile,
  };
}
