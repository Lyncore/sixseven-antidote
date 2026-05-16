<script setup lang="ts">
import { useAntidote } from "./composables/useAntidote";
import ScanControls from "./components/ScanControls.vue";
import FileTable from "./components/FileTable.vue";
import ApplyLog from "./components/ApplyLog.vue";

const {
  scanPath, recursive, backup,
  scanning, applying,
  results, selected, allSelected,
  applyLog, error,
  toggleAll, toggleFile,
  pickFolder, doScan, applyTo, openFile,
} = useAntidote();
</script>

<template>
  <div class="app">
    <header>
      <h1>67 → 69</h1>
      <p class="subtitle">Поиск и замена числа 67 в документах</p>
    </header>

    <ScanControls
      v-model:scanPath="scanPath"
      v-model:recursive="recursive"
      v-model:backup="backup"
      :scanning="scanning"
      :applying="applying"
      @pickFolder="pickFolder"
      @scan="doScan"
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
      Файлы не найдены или поиск ещё не запущен
    </div>

    <ApplyLog v-if="applyLog.length" :log="applyLog" />
  </div>
</template>

<style>
@import "./assets/app.css";
</style>
