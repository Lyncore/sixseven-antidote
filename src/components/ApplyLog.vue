<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { ApplyResult } from "../composables/useAntidote";

const { t } = useI18n();

defineProps<{
  log: ApplyResult[];
}>();
</script>

<template>
  <section class="card log">
    <h3>{{ t("logTitle") }}</h3>
    <div
      v-for="r in log"
      :key="r.path"
      class="log-entry"
      :class="r.success ? 'log-ok' : 'log-err'"
    >
      <span class="log-icon">{{ r.success ? "✓" : "✗" }}</span>
      <span class="log-path">{{ r.path }}</span>
      <span v-if="r.error" class="log-error-msg">{{ r.error }}</span>
    </div>
  </section>
</template>
