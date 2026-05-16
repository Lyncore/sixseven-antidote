import { createI18n } from 'vue-i18n';
import ruRU from './locales/ru-RU.json';
import enUS from './locales/en-US.json';

function detectLocale(): string {
  const saved = localStorage.getItem('locale');
  if (saved) return saved;
  return navigator.language.startsWith('ru') ? 'ru' : 'en';
}

export const i18n = createI18n({
  legacy: false,
  locale: detectLocale(),
  fallbackLocale: 'en',
  messages: {
    en: enUS,
    ru: ruRU,
  },
});
