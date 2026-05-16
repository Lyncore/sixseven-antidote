<script setup lang="ts">
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
      <input v-model="scanPath" placeholder="Путь к папке..." class="path-input" />
      <button @click="emit('pickFolder')" class="btn-secondary">Обзор</button>
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
      <button @click="emit('scan', false)" :disabled="scanning || applying" class="btn-primary">
        {{ scanning ? "Поиск..." : "Сканировать папку" }}
      </button>
      <button @click="emit('scan', true)" :disabled="scanning || applying" class="btn-danger">
        {{ scanning ? "Поиск..." : "Весь ПК" }}
      </button>
    </div>
  </section>
</template>
