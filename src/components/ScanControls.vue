<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ScanProgress } from '../composables/useAntidote';

const { t } = useI18n();

defineProps<{
  scanning: boolean;
  applying: boolean;
  progress: ScanProgress | null;
}>();

const scanPath = defineModel<string>('scanPath', { required: true });
const recursive = defineModel<boolean>('recursive', { required: true });
const backup = defineModel<boolean>('backup', { required: true });

const emit = defineEmits<{
  pickFolder: [];
  scan: [allDrives: boolean];
  cancel: [];
}>();
</script>

<template>
  <section class="card">
    <div class="path-row">
      <input
        v-model="scanPath"
        :placeholder="t('folderPlaceholder')"
        class="path-input"
      />
      <button
        @click="emit('pickFolder')"
        class="btn-secondary"
        :disabled="scanning"
      >
        {{ t('browse') }}
      </button>
    </div>

    <div class="options-row">
      <label class="checkbox-label">
        <input type="checkbox" v-model="recursive" :disabled="scanning" />
        {{ t('recursive') }}
      </label>
      <label class="checkbox-label">
        <input type="checkbox" v-model="backup" :disabled="scanning" />
        {{ t('backups') }}
      </label>
    </div>

    <div class="btn-row">
      <button
        @click="emit('scan', false)"
        :disabled="scanning || applying"
        class="btn-primary"
      >
        {{ t('scanFolder') }}
      </button>
      <button
        @click="emit('scan', true)"
        :disabled="scanning || applying"
        class="btn-danger"
      >
        {{ t('scanPC') }}
      </button>
      <button v-if="scanning" @click="emit('cancel')" class="btn-secondary">
        {{ t('cancel') }}
      </button>
    </div>

    <div v-if="scanning" class="progress-wrap">
      <div class="progress-bar-track">
        <div
          class="progress-bar-fill"
          :style="{ width: (progress?.percent ?? 0) + '%' }"
        ></div>
      </div>
      <div class="progress-info">
        <span class="progress-percent">{{ progress?.percent ?? 0 }}%</span>
        <span v-if="!progress">{{ t('progressPrepare') }}</span>
        <span v-if="progress" class="progress-stats">
          {{
            t('progressScanned', {
              scanned: progress.scanned,
              total: progress.total,
            })
          }}
          &nbsp;·&nbsp;
          {{ t('progressFound', { found: progress.found }) }}
        </span>
        <span v-if="progress" class="progress-current">{{
          progress.current
        }}</span>
      </div>
    </div>
  </section>
</template>
