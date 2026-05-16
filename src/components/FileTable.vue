<script setup lang="ts">
import type { FileMatch } from "../composables/useAntidote";

const props = defineProps<{
  results: FileMatch[];
  selected: Set<string>;
  allSelected: boolean;
  applying: boolean;
}>();

const emit = defineEmits<{
  toggleAll: [];
  toggleFile: [path: string];
  applySelected: [];
  applyAll: [];
  openFile: [path: string];
}>();
</script>

<template>
  <section class="card">
    <div class="results-header">
      <span>Найдено файлов: {{ results.length }}</span>
      <div class="btn-row">
        <button
          @click="emit('applySelected')"
          :disabled="applying || !selected.size"
          class="btn-primary"
        >
          {{ applying ? "Применяю..." : `Заменить выбранные (${selected.size})` }}
        </button>
        <button @click="emit('applyAll')" :disabled="applying" class="btn-danger">
          {{ applying ? "Применяю..." : "Заменить все" }}
        </button>
      </div>
    </div>

    <table class="file-table">
      <thead>
        <tr>
          <th><input type="checkbox" :checked="allSelected" @change="emit('toggleAll')" /></th>
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
          @click="emit('toggleFile', file.path)"
        >
          <td @click.stop>
            <input
              type="checkbox"
              :checked="selected.has(file.path)"
              @change="emit('toggleFile', file.path)"
            />
          </td>
          <td class="path-cell" :title="file.path">{{ file.path }}</td>
          <td class="count-cell">{{ file.match_count }}</td>
          <td class="format-cell">{{ file.format.toUpperCase() }}</td>
          <td class="actions-cell" @click.stop>
            <button @click="emit('openFile', file.path)" class="btn-small">Открыть</button>
          </td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
