<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { FileMatch } from '../composables/useAntidote';

const { t } = useI18n();

defineProps<{
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
  <section class="card results-card">
    <div class="results-header">
      <span>{{ t('filesFound', { count: results.length }) }}</span>
      <div class="btn-row">
        <button
          @click="emit('applySelected')"
          :disabled="applying || !selected.size"
          class="btn-primary"
        >
          {{
            applying
              ? t('applying')
              : t('replaceSelected', { count: selected.size })
          }}
        </button>
        <button
          @click="emit('applyAll')"
          :disabled="applying"
          class="btn-danger"
        >
          {{ applying ? t('applying') : t('replaceAll') }}
        </button>
      </div>
    </div>

    <div class="file-table-scroll">
      <table class="file-table">
        <thead>
          <tr>
            <th>
              <input
                type="checkbox"
                :checked="allSelected"
                @change="emit('toggleAll')"
              />
            </th>
            <th>{{ t('colFile') }}</th>
            <th>{{ t('colMatches') }}</th>
            <th>{{ t('colFormat') }}</th>
            <th>{{ t('colActions') }}</th>
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
              <button @click="emit('openFile', file.path)" class="btn-small">
                {{ t('open') }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>
