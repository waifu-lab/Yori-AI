# 前端設計規範

## 1. 技術架構

### 1.1 技術棧

- **框架**: Svelte 5 + SvelteKit
- **樣式**: Tailwind CSS
- **調色方案**: Tailwind CSS 預設色

### 1.2 支援平台

- 直播模式：僅支援電腦
- 桌面寵物模式：僅支援電腦

---

## 2. 調色方案

### 2.1 主要色彩

使用 Tailwind CSS 預設色系：

```css
@theme {
  --color-background: oklch(0.15 0.02 250);
  --color-foreground: oklch(0.9 0.01 250);
  --color-muted: oklch(0.25 0.02 250);
  --color-muted-foreground: oklch(0.6 0.01 250);
  --color-popover: oklch(0.15 0.02 250);
  --color-popover-foreground: oklch(0.7 0.01 250);
  --color-card: oklch(0.15 0.02 250);
  --color-card-foreground: oklch(0.9 0.01 250);
  --color-border: oklch(0.3 0.02 250);
  --color-input: oklch(0.3 0.02 250);
  --color-primary: oklch(0.98 0.005 250);
  --color-primary-foreground: oklch(0.2 0.02 250);
  --color-secondary: oklch(0.2 0.02 250);
  --color-secondary-foreground: oklch(0.98 0.005 250);
  --color-accent: oklch(0.3 0.02 250);
  --color-accent-foreground: oklch(0.98 0.005 250);
  --color-destructive: oklch(0.45 0.15 30);
  --color-destructive-foreground: oklch(0.98 0.005 250);
  --color-ring: oklch(0.3 0.02 250);

  --radius-sm: 0.25rem;
  --radius-md: 0.375rem;
  --radius-lg: 0.5rem;
}
```

### 2.2 深淺模式

- **預設**: 深色模式（適合直播場景）
- **可選**: 淺色模式
- 使用 Tailwind `dark:` 修飾符

---

## 3. 可及性 (Accessibility)

### 3.1 鍵盤導航

```svelte
<!-- 確保所有互動元素可通過鍵盤訪問 -->
<button
  on:keydown={(e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleClick();
    }
  }}
>
```

### 3.2 ARIA 標籤

```svelte
<aside
  role="dialog"
  aria-label="設定面板"
  aria-modal="true"
>
  ...
</aside>

<button
  aria-expanded={isOpen}
  aria-controls="settings-panel"
>
  設定
</button>
```

### 3.3 焦點管理

```svelte
<!-- 打開 Modal 時自動聚焦 -->
<script>
  let closeButton: HTMLButtonElement;

  $effect(() => {
    if (open && closeButton) {
      closeButton.focus();
    }
  });
</script>
```

---

## 4. 錯誤處理

### 4.1 錯誤邊界

```svelte
<!-- ErrorBoundary.svelte -->
<script>
  import { onMount } from 'svelte';

  let error = $state<Error | null>(null);

  onMount(() => {
    window.addEventListener('error', (e) => {
      error = e.error;
    });
  });
</script>

{#if error}
  <div class="p-4 bg-red-900/50 border border-red-700 rounded-lg">
    <h3 class="text-red-400 font-medium">發生錯誤</h3>
    <p class="text-red-300 text-sm mt-1">{error.message}</p>
    <button
      class="mt-2 text-sm text-red-400 hover:text-red-300"
      on:click={() => (error = null)}
    >
      關閉
    </button>
  </div>
{:else}
  <slot />
{/if}
```

### 4.2 載入狀態

```svelte
<script>
  let isLoading = $state(false);
  let loadError = $state<string | null>(null);
</script>

{#if isLoading}
  <div class="flex items-center justify-center p-8">
    <LoadingSpinner />
    <span class="ml-3 text-slate-400">載入中...</span>
  </div>
{:else if loadError}
  <div class="p-4 bg-red-900/30 rounded-lg text-center">
    <p class="text-red-400">{loadError}</p>
    <button
      class="mt-2 text-sm text-indigo-400 hover:text-indigo-300"
      on:click={retry}
    >
      重試
    </button>
  </div>
{:else}
  <slot />
{/if}
```

---

## 5. 國際化準備 (i18n Ready)

使用Paraglide JS
