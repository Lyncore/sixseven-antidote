<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useAntidote } from "./composables/useAntidote";
import ScanControls from "./components/ScanControls.vue";
import FileTable from "./components/FileTable.vue";
import ApplyLog from "./components/ApplyLog.vue";

const { t, locale } = useI18n();

function onLocaleChange(e: Event) {
  const val = (e.target as HTMLSelectElement).value;
  locale.value = val;
  localStorage.setItem("locale", val);
}

const {
  scanPath, recursive, backup,
  scanning, applying, progress,
  results, selected, allSelected,
  applyLog, error,
  toggleAll, toggleFile,
  pickFolder, doScan, cancelScan, applyTo, openFile,
} = useAntidote();
</script>

<template>
  <div class="app">
    <header>
      <div class="header-main">
        <img src="/logo.svg" width="64" height="64"/>
        <div class="header-title">
          <h1>Sixseven Antidote</h1>
          <p class="subtitle">{{ t("subtitle") }}</p>
        </div>
        <select class="lang-select" :value="locale" @change="onLocaleChange">
          <option value="ru">Русский</option>
          <option value="en">English</option>
        </select>
      </div>
    </header>

    <ScanControls
      v-model:scanPath="scanPath"
      v-model:recursive="recursive"
      v-model:backup="backup"
      :scanning="scanning"
      :applying="applying"
      :progress="progress"
      @pickFolder="pickFolder"
      @scan="doScan"
      @cancel="cancelScan"
    />

    <div v-if="error" class="error-box">{{ error }}</div>

    <FileTable
      v-if="results.length"
      :results="results"
      :selected="selected"
      :allSelected="allSelected"
      :applying="applying"
      @toggleAll="toggleAll"
      @toggleFile="toggleFile"
      @applySelected="applyTo([...selected])"
      @applyAll="applyTo(results.map((f) => f.path))"
      @openFile="openFile"
    />

    <div v-else-if="!scanning && !applyLog.length" class="empty">
      {{ t("empty") }}
    </div>

    <ApplyLog v-if="applyLog.length" :log="applyLog" />
  </div>
</template>

<style>
@import "./assets/app.css";
</style>
