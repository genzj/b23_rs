# Track: Frontend i18n
## Overview
Add Internationalization (i18n) support to the `index.html` frontend, initially supporting English and Simplified Chinese (zh-cn), with an extensible architecture for future languages. The translation data will be maintained in separate JSON files to allow easy GitHub PR collaboration, and the backend will inject the appropriate translations into the HTML based on request headers/cookies.

## Functional Requirements
1. **Language Detection & Fallback**: The server should determine the language based on the `Cookie`, `Accept-Language` header, or other browser info on the first visit. It should fallback to English if the preferred language is unsupported.
2. **Translation Storage**: Translation texts will be stored in individual JSON files (e.g., `locales/en.json`, `locales/zh-cn.json`).
3. **Build/Server Integration**: The build process or the Rust backend server should combine the translations and inject the required language translations directly into the HTML response of the index handler, avoiding client-side network requests for translations.
4. **Language Switcher UI**: Provide a dropdown menu for language selection, utilizing a flag emoji and short name (e.g., "🇺🇸 EN", "🇨🇳 中文").
5. **Preference Persistence**: The user's selected language preference must be saved in a Cookie, which the server will read on subsequent requests.
6. **Comprehensive Coverage**: All page content, including static texts, button labels, input placeholders, and page titles, must be translated.
7. **Brand Consistency**: Terminologies and brand names (e.g., `b23`, `b23.wtf`, `b23.rust`, `rust`) will remain in English across all translations.

## Non-Functional Requirements
1. **Collaboration**: JSON files should be structured simply to encourage open-source contributions for new languages via GitHub PRs.
2. **Performance**: Since the Rust backend is injecting the translations, the client-side JS remains lightweight and immediately renders the correct language.

## Acceptance Criteria
- Loading the page with a Chinese browser (or `zh-CN` in `Accept-Language`) displays the Chinese UI.
- Loading with English (or unsupported language) displays English UI.
- User can change the language via the dropdown, which sets a Cookie and updates the UI immediately or via page reload.
- The Rust build process/binary includes the JSON files and serves the dynamic HTML.
- Brand names are not translated.

## Out of Scope
- Backend API response internationalization (APIs primarily deal with URLs and standard errors).
