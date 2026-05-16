import { createI18n } from "vue-i18n";
import ru from "./ru";
import en from "./en";

function detectLocale(): string {
  const saved = localStorage.getItem("locale");
  if (saved) return saved;
  return navigator.language.startsWith("ru") ? "ru" : "en";
}

export const i18n = createI18n({
  legacy: false,
  locale: detectLocale(),
  fallbackLocale: "en",
  messages: { ru, en },
});
