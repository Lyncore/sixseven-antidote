<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open as dialogOpen } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";

interface FileMatch {
  path: string;
  match_count: number;
  format: string;
}

interface ApplyResult {
  path: string;
  success: boolean;
  error: string | null;
}

const scanPath = ref("");
const recursive = ref(true);
const backup = ref(true);
const scanning = ref(false);
const applying = ref(false);
const results = ref<FileMatch[]>([]);
const selected = ref<Set<string>>(new Set());
const previewPath = ref<string | null>(null);
const previewContent = ref("");
const previewLoading = ref(false);
const applyLog = ref<ApplyResult[]>([]);
const error = ref("");

const allSelected = computed(
  () => results.value.length > 0 && selected.value.size === results.value.length
);

function toggleAll() {
  if (allSelected.value) {
    selected.value = new Set();
  } else {
    selected.value = new Set(results.value.map((f) => f.path));
  }
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
  error.value = "";
  results.value = [];
  selected.value = new Set();
  applyLog.value = [];
  scanning.value = true;
  try {
    if (allDrives) {
      results.value = await invoke<FileMatch[]>("scan_all_drives");
    } else {
      if (!scanPath.value) { error.value = "Укажите папку"; return; }
      results.value = await invoke<FileMatch[]>("scan_directory", {
        path: scanPath.value,
        recursive: recursive.value,
      });
    }
  } catch (e: any) {
    error.value = String(e);
  } finally {
    scanning.value = false;
  }
}

async function applyTo(paths: string[]) {
  if (!paths.length) return;
  applying.value = true;
  applyLog.value = [];
  try {
    applyLog.value = await invoke<ApplyResult[]>("apply_replacements", {
      paths,
      backup: backup.value,
    });
    // Remove successfully replaced files from results
    const done = new Set(applyLog.value.filter((r) => r.success).map((r) => r.path));
    results.value = results.value.filter((f) => !done.has(f.path));
    selected.value = new Set([...selected.value].filter((p) => !done.has(p)));
  } catch (e: any) {
    error.value = String(e);
  } finally {
    applying.value = false;
  }
}

async function showPreview(path: string) {
  previewPath.value = path;
  previewContent.value = "";
  previewLoading.value = true;
  try {
    previewContent.value = await invoke<string>("read_file_content", { path });
  } catch (e: any) {
    previewContent.value = "Ошибка: " + String(e);
  } finally {
    previewLoading.value = false;
  }
}

async function openInApp(path: string) {
  try {
    await openPath(path);
  } catch (e: any) {
    error.value = String(e);
  }
}

function highlightMatches(text: string): string {
  return text.replace(/67/g, '<mark>67</mark>');
}
</script>

<template>
  <div class="app">
    <header>
      <h1>67 → 69</h1>
      <p class="subtitle">Поиск и замена числа 67 в документах</p>
    </header>

    <section class="controls card">
      <div class="path-row">
        <input v-model="scanPath" placeholder="Путь к папке..." class="path-input" />
        <button @click="pickFolder" class="btn-secondary">Обзор</button>
      </div>
      <div class="options-row">
        <label class="checkbox-label">
          <input type="checkbox" v-model="recursive" /> Рекурсивно
        </label>
        <label class="checkbox-label">
          <input type="checkbox" v-model="backup" /> Резервные копии (.bak)
        </label>
      </div>
      <div class="btn-row">
        <button @click="doScan(false)" :disabled="scanning" class="btn-primary">
          {{ scanning ? "Поиск..." : "Сканировать папку" }}
        </button>
        <button @click="doScan(true)" :disabled="scanning" class="btn-danger">
          {{ scanning ? "Поиск..." : "Весь ПК" }}
        </button>
      </div>
    </section>

    <div v-if="error" class="error-box">{{ error }}</div>

    <section v-if="results.length" class="results card">
      <div class="results-header">
        <span>Найдено файлов: {{ results.length }}</span>
        <div class="btn-row">
          <button
            @click="applyTo([...selected])"
            :disabled="applying || !selected.size"
            class="btn-primary"
          >
            {{ applying ? "Применяю..." : `Заменить выбранные (${selected.size})` }}
          </button>
          <button
            @click="applyTo(results.map((f) => f.path))"
            :disabled="applying"
            class="btn-danger"
          >
            {{ applying ? "Применяю..." : "Заменить все" }}
          </button>
        </div>
      </div>

      <table class="file-table">
        <thead>
          <tr>
            <th><input type="checkbox" :checked="allSelected" @change="toggleAll" /></th>
            <th>Файл</th>
            <th>Совпадений</th>
            <th>Формат</th>
            <th>Действия</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="file in results"
            :key="file.path"
            :class="{ selected: selected.has(file.path) }"
            @click="toggleFile(file.path)"
          >
            <td @click.stop>
              <input
                type="checkbox"
                :checked="selected.has(file.path)"
                @change="toggleFile(file.path)"
              />
            </td>
            <td class="path-cell" :title="file.path">{{ file.path }}</td>
            <td class="count-cell">{{ file.match_count }}</td>
            <td class="format-cell">{{ file.format.toUpperCase() }}</td>
            <td class="actions-cell" @click.stop>
              <button @click="showPreview(file.path)" class="btn-small">Просмотр</button>
              <button @click="openInApp(file.path)" class="btn-small">Открыть</button>
            </td>
          </tr>
        </tbody>
      </table>
    </section>

    <div v-else-if="!scanning && results.length === 0 && applyLog.length === 0" class="empty">
      Файлы не найдены или поиск ещё не запущен
    </div>

    <section v-if="applyLog.length" class="log card">
      <h3>Результаты замены</h3>
      <div
        v-for="r in applyLog"
        :key="r.path"
        class="log-entry"
        :class="r.success ? 'log-ok' : 'log-err'"
      >
        <span class="log-icon">{{ r.success ? "✓" : "✗" }}</span>
        <span class="log-path">{{ r.path }}</span>
        <span v-if="r.error" class="log-error-msg">{{ r.error }}</span>
      </div>
    </section>

    <section v-if="previewPath" class="preview card">
      <div class="preview-header">
        <h3>Просмотр: {{ previewPath }}</h3>
        <button @click="previewPath = null" class="btn-small">Закрыть</button>
      </div>
      <div v-if="previewLoading" class="preview-loading">Загрузка...</div>
      <pre
        v-else
        class="preview-content"
        v-html="highlightMatches(previewContent)"
      ></pre>
    </section>
  </div>
</template>

<style>
* { box-sizing: border-box; margin: 0; padding: 0; }

:root {
  font-family: Inter, system-ui, sans-serif;
  font-size: 14px;
  --bg: #f0f0f0;
  --card-bg: #ffffff;
  --border: #d0d0d0;
  --primary: #2563eb;
  --primary-hover: #1d4ed8;
  --danger: #dc2626;
  --danger-hover: #b91c1c;
  --text: #111;
  --text-muted: #666;
  --selected-bg: #eff6ff;
  --mark: #fde68a;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #1a1a1a;
    --card-bg: #242424;
    --border: #3a3a3a;
    --text: #f0f0f0;
    --text-muted: #999;
    --selected-bg: #1e3a5f;
    --mark: #7c5f00;
  }
}

body { background: var(--bg); color: var(--text); }

.app {
  max-width: 960px;
  margin: 0 auto;
  padding: 24px 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

header { text-align: center; }
header h1 { font-size: 2rem; font-weight: 700; }
.subtitle { color: var(--text-muted); margin-top: 4px; }

.card {
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 16px;
}

.path-row {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}
.path-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg);
  color: var(--text);
  font-size: 14px;
}
.options-row {
  display: flex;
  gap: 20px;
  margin-bottom: 12px;
}
.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}

.btn-row {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

button {
  border: none;
  border-radius: 6px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.15s;
}
button:disabled { opacity: 0.5; cursor: not-allowed; }

.btn-primary { background: var(--primary); color: #fff; }
.btn-primary:hover:not(:disabled) { background: var(--primary-hover); }
.btn-danger { background: var(--danger); color: #fff; }
.btn-danger:hover:not(:disabled) { background: var(--danger-hover); }
.btn-secondary { background: var(--border); color: var(--text); }
.btn-secondary:hover:not(:disabled) { filter: brightness(0.9); }
.btn-small { padding: 4px 10px; font-size: 12px; background: var(--border); color: var(--text); }
.btn-small:hover { filter: brightness(0.9); }

.error-box {
  background: #fee2e2;
  color: #991b1b;
  border: 1px solid #fca5a5;
  border-radius: 8px;
  padding: 12px;
}

.results-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
  flex-wrap: wrap;
  gap: 8px;
}

.file-table {
  width: 100%;
  border-collapse: collapse;
}
.file-table th {
  text-align: left;
  padding: 8px 10px;
  border-bottom: 2px solid var(--border);
  font-weight: 600;
  color: var(--text-muted);
  font-size: 12px;
  text-transform: uppercase;
}
.file-table td {
  padding: 8px 10px;
  border-bottom: 1px solid var(--border);
  vertical-align: middle;
}
.file-table tr:last-child td { border-bottom: none; }
.file-table tbody tr { cursor: pointer; transition: background 0.1s; }
.file-table tbody tr:hover { background: var(--selected-bg); }
.file-table tbody tr.selected { background: var(--selected-bg); }

.path-cell {
  max-width: 420px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  font-family: monospace;
}
.count-cell { text-align: center; font-weight: 600; color: var(--danger); }
.format-cell { font-size: 12px; color: var(--text-muted); }
.actions-cell { display: flex; gap: 6px; }

.empty { text-align: center; color: var(--text-muted); padding: 32px; }

.log h3 { margin-bottom: 10px; }
.log-entry {
  display: flex;
  align-items: baseline;
  gap: 8px;
  padding: 5px 0;
  border-bottom: 1px solid var(--border);
  font-size: 13px;
}
.log-entry:last-child { border-bottom: none; }
.log-ok .log-icon { color: #16a34a; font-weight: 700; }
.log-err .log-icon { color: var(--danger); font-weight: 700; }
.log-path { font-family: monospace; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.log-error-msg { color: var(--danger); font-size: 12px; }

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.preview-header h3 {
  font-size: 13px;
  font-family: monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.preview-loading { color: var(--text-muted); padding: 16px; text-align: center; }
.preview-content {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 12px;
  font-family: monospace;
  font-size: 13px;
  overflow: auto;
  max-height: 360px;
  white-space: pre-wrap;
  word-break: break-word;
}
mark { background: var(--mark); border-radius: 2px; padding: 0 2px; }
</style>
