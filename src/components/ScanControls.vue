<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();

defineProps<{
  scanning: boolean;
  applying: boolean;
}>();

const scanPath = defineModel<string>("scanPath", { required: true });
const recursive = defineModel<boolean>("recursive", { required: true });
const backup = defineModel<boolean>("backup", { required: true });

const emit = defineEmits<{
  pickFolder: [];
  scan: [allDrives: boolean];
}>();
</script>

<template>
  <section class="card">
    <div class="path-row">
      <input v-model="scanPath" :placeholder="t('folderPlaceholder')" class="path-input" />
      <button @click="emit('pickFolder')" class="btn-secondary">{{ t("browse") }}</button>
    </div>

    <div class="options-row">
      <label class="checkbox-label">
        <input type="checkbox" v-model="recursive" /> {{ t("recursive") }}
      </label>
      <label class="checkbox-label">
        <input type="checkbox" v-model="backup" /> {{ t("backups") }}
      </label>
    </div>

    <div class="btn-row">
      <button @click="emit('scan', false)" :disabled="scanning || applying" class="btn-primary">
        {{ scanning ? t("searching") : t("scanFolder") }}
      </button>
      <button @click="emit('scan', true)" :disabled="scanning || applying" class="btn-danger">
        {{ scanning ? t("searching") : t("scanPC") }}
      </button>
    </div>
  </section>
</template>
