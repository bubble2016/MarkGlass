<template>
  <main
    class="app-shell"
    :class="[themeClass, { 'is-dragging': isDragging, 'is-immersive': isImmersive }]"
    @click="closeContextMenu"
    @contextmenu.prevent="openContextMenu"
  >
    <header v-if="isImmersive" class="titlebar" data-tauri-drag-region>
      <div class="titlebar-brand" data-tauri-drag-region>
        <img :src="markGlassIcon" alt="" />
        <span data-tauri-drag-region>{{ currentFileName || 'MarkGlass' }}</span>
      </div>
      <div class="window-actions">
        <button type="button" aria-label="最小化窗口" title="最小化" @click.stop="minimizeWindow">
          <Minus :size="16" />
        </button>
        <button type="button" aria-label="最大化或还原窗口" title="最大化或还原" @click.stop="toggleMaximize">
          <Square :size="14" />
        </button>
        <button class="close-button" type="button" aria-label="关闭窗口" title="关闭" @click.stop="closeWindow">
          <X :size="17" />
        </button>
      </div>
    </header>

    <section v-if="isLoading && !html" class="empty-state status-state" aria-live="polite">
      <LoaderCircle class="spin" :size="34" />
      <p>正在打开 Markdown…</p>
    </section>

    <section v-else-if="errorMessage && !html" class="empty-state status-state" role="alert">
      <AlertCircle :size="38" />
      <h1>无法打开文件</h1>
      <p>{{ errorMessage }}</p>
      <button class="primary-button" type="button" @click="openFile">
        <FolderOpen :size="18" />
        选择其他文件
      </button>
    </section>

    <section v-else-if="!html" class="empty-state">
      <img class="app-logo" :src="markGlassIcon" alt="MarkGlass 图标" />
      <p class="eyebrow">MARKDOWN VIEWER</p>
      <h1>MarkGlass</h1>
      <p class="empty-description">双击 Markdown 文件，或把文档拖到这里，即刻开始阅读。</p>
      <button class="primary-button" type="button" @click="openFile">
        <FolderOpen :size="18" />
        打开 Markdown
      </button>
      <div class="hint">
        <span><kbd>Ctrl</kbd> + <kbd>O</kbd> 打开</span>
        <span><kbd>F11</kbd> 全屏</span>
        <span><kbd>←</kbd> <kbd>→</kbd> 连续阅读</span>
      </div>
    </section>

    <section v-else class="reader-wrap" :aria-busy="isLoading">
      <article ref="articleRef" class="markdown-body" v-html="html"></article>

      <div v-if="errorMessage" class="inline-error" role="alert">
        <AlertCircle :size="17" />
        <span>{{ errorMessage }}</span>
        <button type="button" aria-label="关闭错误提示" @click="errorMessage = ''">
          <X :size="15" />
        </button>
      </div>

      <nav class="floating-bar" aria-label="阅读工具栏">
        <button
          type="button"
          aria-label="上一篇"
          title="上一篇（←）"
          :disabled="!canOpenPrevious"
          @click="openRelative(-1)"
        >
          <ChevronLeft :size="18" />
        </button>
        <span class="file-name" :title="currentPath">{{ currentFileName }}</span>
        <button
          type="button"
          aria-label="下一篇"
          title="下一篇（→）"
          :disabled="!canOpenNext"
          @click="openRelative(1)"
        >
          <ChevronRight :size="18" />
        </button>
        <span class="toolbar-divider" aria-hidden="true"></span>
        <button type="button" aria-label="打开文件" title="打开文件（Ctrl+O）" @click="openFile">
          <FolderOpen :size="18" />
        </button>
        <button type="button" :aria-label="`当前主题：${themeLabel}`" :title="`主题：${themeLabel}`" @click="toggleTheme">
          <MonitorCog v-if="theme === 'auto'" :size="18" />
          <Sun v-else-if="theme === 'light'" :size="18" />
          <Moon v-else :size="18" />
        </button>
        <button type="button" aria-label="切换沉浸窗口" title="无边框沉浸窗口" @click="toggleImmersive">
          <PanelsTopLeft :size="18" />
        </button>
        <button type="button" aria-label="切换全屏" title="全屏（F11）" @click="toggleFullscreen">
          <Maximize2 :size="18" />
        </button>
      </nav>
    </section>

    <div
      v-if="contextMenu.visible"
      ref="contextMenuRef"
      class="context-menu"
      role="menu"
      :style="{ left: `${contextMenu.x}px`, top: `${contextMenu.y}px` }"
      @click.stop
    >
      <button type="button" role="menuitem" @click="runMenuAction(copySelection)">
        <Copy :size="16" />
        <span>复制所选内容</span>
        <kbd>Ctrl+C</kbd>
      </button>
      <button type="button" role="menuitem" @click="runMenuAction(openFile)">
        <FolderOpen :size="16" />
        <span>打开文件</span>
        <kbd>Ctrl+O</kbd>
      </button>
      <div class="menu-separator" role="separator"></div>
      <button type="button" role="menuitem" :disabled="!canOpenPrevious" @click="runMenuAction(() => openRelative(-1))">
        <ChevronLeft :size="16" />
        <span>上一篇</span>
        <kbd>←</kbd>
      </button>
      <button type="button" role="menuitem" :disabled="!canOpenNext" @click="runMenuAction(() => openRelative(1))">
        <ChevronRight :size="16" />
        <span>下一篇</span>
        <kbd>→</kbd>
      </button>
      <div class="menu-separator" role="separator"></div>
      <button type="button" role="menuitem" @click="runMenuAction(toggleTheme)">
        <Palette :size="16" />
        <span>切换主题</span>
        <small>{{ themeLabel }}</small>
      </button>
      <button type="button" role="menuitem" @click="runMenuAction(toggleImmersive)">
        <PanelsTopLeft :size="16" />
        <span>沉浸窗口</span>
        <Check v-if="isImmersive" :size="15" />
      </button>
      <button type="button" role="menuitem" @click="runMenuAction(toggleFullscreen)">
        <Maximize2 :size="16" />
        <span>全屏</span>
        <kbd>F11</kbd>
      </button>
    </div>

    <div v-if="toastMessage" class="toast" role="status">{{ toastMessage }}</div>

    <div v-if="isDragging" class="drop-overlay" aria-hidden="true">
      <FileDown :size="42" />
      <strong>松开以打开 Markdown</strong>
      <span>支持 .md、.mdx 和 .markdown</span>
    </div>
  </main>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import MarkdownIt from 'markdown-it'
import anchor from 'markdown-it-anchor'
import footnote from 'markdown-it-footnote'
import taskLists from 'markdown-it-task-lists'
import mermaid from 'mermaid'
import { createHighlighter } from 'shiki'
import {
  AlertCircle,
  Check,
  ChevronLeft,
  ChevronRight,
  Copy,
  FileDown,
  FolderOpen,
  LoaderCircle,
  Maximize2,
  Minus,
  MonitorCog,
  Moon,
  Palette,
  PanelsTopLeft,
  Square,
  Sun,
  X
} from '@lucide/vue'
import markGlassIcon from './assets/markglass-icon.png'

interface MarkdownDocument {
  path: string
  content: string
  siblings: string[]
}

type Theme = 'auto' | 'light' | 'dark'

const isTauri = '__TAURI_INTERNALS__' in window
const appWindow = isTauri ? getCurrentWindow() : null
const html = ref('')
const currentSource = ref('')
const currentPath = ref('')
const siblingFiles = ref<string[]>([])
const isDragging = ref(false)
const isLoading = ref(false)
const isImmersive = ref(isTauri && localStorage.getItem('markglass-immersive') === 'true')
const errorMessage = ref('')
const toastMessage = ref('')
const articleRef = ref<HTMLElement | null>(null)
const contextMenuRef = ref<HTMLElement | null>(null)
const theme = ref<Theme>((localStorage.getItem('markglass-theme') as Theme) || 'auto')
const systemTheme = window.matchMedia('(prefers-color-scheme: dark)')
const systemIsDark = ref(systemTheme.matches)
const contextMenu = reactive({ visible: false, x: 0, y: 0 })

const demoMarkdown = `# 欢迎使用 MarkGlass

MarkGlass 是一个专注于阅读体验的 Windows Markdown 阅读器。

> 拖入文档、双击文件，或者使用 **Ctrl + O** 即可开始阅读。

## 代码高亮

\`\`\`typescript
interface Reader {
  name: string
  immersive: boolean
}

const markGlass: Reader = {
  name: 'MarkGlass',
  immersive: true
}
\`\`\`

## Mermaid 图表

\`\`\`mermaid
flowchart LR
  A[打开 Markdown] --> B[解析内容]
  B --> C[Shiki 高亮]
  B --> D[Mermaid 图表]
  C --> E[沉浸阅读]
  D --> E
\`\`\`

## 支持情况

| 能力 | 状态 |
| --- | --- |
| 亮色 / 暗色 | 已支持 |
| 同目录连续阅读 | 已支持 |
| 无边框沉浸窗口 | 已支持 |
`

let toastTimer: number | undefined
let renderSequence = 0
let unlistenOpenFile: UnlistenFn | undefined
let unlistenDragDrop: UnlistenFn | undefined

const themeClass = computed(() => `theme-${theme.value}`)
const themeLabel = computed(() => (theme.value === 'auto' ? '跟随系统' : theme.value === 'light' ? '亮色' : '暗色'))
const currentFileName = computed(() => currentPath.value.split(/[\\/]/).pop() || '')
const currentSiblingIndex = computed(() =>
  siblingFiles.value.findIndex((path) => path.toLowerCase() === currentPath.value.toLowerCase())
)
const canOpenPrevious = computed(() => currentSiblingIndex.value > 0)
const canOpenNext = computed(
  () => currentSiblingIndex.value >= 0 && currentSiblingIndex.value < siblingFiles.value.length - 1
)
const effectiveDark = computed(() => theme.value === 'dark' || (theme.value === 'auto' && systemIsDark.value))

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true
})
  .use(anchor, { permalink: anchor.permalink.linkInsideHeader({ symbol: '#' }) })
  .use(footnote)
  .use(taskLists, { enabled: true, label: true })

const highlighterPromise = createHighlighter({
  themes: ['github-light', 'github-dark'],
  langs: [
    'bash',
    'c',
    'cpp',
    'css',
    'diff',
    'html',
    'java',
    'javascript',
    'json',
    'jsx',
    'markdown',
    'powershell',
    'python',
    'rust',
    'sql',
    'typescript',
    'tsx',
    'vue',
    'yaml'
  ]
})

function normalizeFilePath(path: string) {
  let value = path.trim().replace(/^["']|["']$/g, '')
  if (value.startsWith('file://')) {
    try {
      value = decodeURIComponent(new URL(value).pathname)
    } catch {
      value = value.replace(/^file:\/\//, '')
    }
  }
  return value.replace(/^\/([A-Za-z]:)/, '$1')
}

function extname(path: string) {
  return path.split('.').pop()?.toLowerCase() || ''
}

function isMarkdown(path: string) {
  return ['md', 'mdx', 'markdown'].includes(extname(path))
}

function normalizeLanguage(language: string) {
  const aliases: Record<string, string> = {
    js: 'javascript',
    ts: 'typescript',
    shell: 'bash',
    sh: 'bash',
    ps1: 'powershell',
    py: 'python',
    rs: 'rust',
    yml: 'yaml',
    md: 'markdown',
    text: 'plaintext',
    txt: 'plaintext'
  }
  return aliases[language] || language || 'plaintext'
}

async function renderCodeBlocks(sequence: number) {
  const article = articleRef.value
  if (!article) return

  const highlighter = await highlighterPromise
  if (sequence !== renderSequence) return

  const blocks = Array.from(article.querySelectorAll('pre > code')).filter(
    (code) => !code.classList.contains('language-mermaid')
  )

  for (const code of blocks) {
    const source = code.textContent || ''
    const className = Array.from(code.classList).find((name) => name.startsWith('language-'))
    const language = normalizeLanguage(className?.replace('language-', '') || '')
    let highlighted: string

    try {
      highlighted = highlighter.codeToHtml(source, {
        lang: language,
        themes: { light: 'github-light', dark: 'github-dark' },
        defaultColor: false
      })
    } catch {
      highlighted = highlighter.codeToHtml(source, {
        lang: 'plaintext',
        themes: { light: 'github-light', dark: 'github-dark' },
        defaultColor: false
      })
    }

    if (sequence !== renderSequence) return
    const wrapper = document.createElement('div')
    wrapper.className = 'code-block'
    wrapper.innerHTML = highlighted
    code.parentElement?.replaceWith(wrapper)
  }
}

async function renderMermaid(sequence: number) {
  const article = articleRef.value
  if (!article) return

  mermaid.initialize({
    startOnLoad: false,
    securityLevel: 'strict',
    theme: effectiveDark.value ? 'dark' : 'default',
    fontFamily: '"Segoe UI", "Microsoft YaHei", sans-serif'
  })

  const blocks = article.querySelectorAll('code.language-mermaid')
  let index = 0

  for (const code of Array.from(blocks)) {
    const source = code.textContent || ''
    const container = document.createElement('div')
    container.className = 'mermaid-render'
    try {
      const { svg } = await mermaid.render(`markglass-mermaid-${sequence}-${index++}`, source)
      if (sequence !== renderSequence) return
      container.innerHTML = svg
    } catch (error) {
      container.classList.add('mermaid-error')
      container.textContent = `Mermaid 图表渲染失败\n${error instanceof Error ? error.message : String(error)}`
    }
    code.parentElement?.replaceWith(container)
  }
}

async function renderDocument(content: string) {
  const sequence = ++renderSequence
  html.value = md.render(content)
  await nextTick()
  await Promise.all([renderCodeBlocks(sequence), renderMermaid(sequence)])
}

async function loadFile(path: string) {
  const normalizedPath = normalizeFilePath(path)
  if (!isMarkdown(normalizedPath)) {
    showToast('请选择 Markdown 文件')
    return
  }

  isLoading.value = true
  errorMessage.value = ''

  try {
    if (!isTauri) throw new Error('网页预览不能读取本地文件，请在 MarkGlass 桌面应用中打开。')
    const document = await invoke<MarkdownDocument>('open_markdown', { path: normalizedPath })
    currentPath.value = document.path
    currentSource.value = document.content
    siblingFiles.value = document.siblings
    await renderDocument(document.content)
    window.scrollTo({ top: 0, behavior: 'instant' })
    window.document.title = `${currentFileName.value} — MarkGlass`
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : String(error)
  } finally {
    isLoading.value = false
  }
}

async function openFile() {
  if (!isTauri) {
    showToast('请在 MarkGlass 桌面应用中打开文件')
    return
  }

  const selected = await open({
    multiple: false,
    title: '使用 MarkGlass 打开 Markdown',
    filters: [{ name: 'Markdown', extensions: ['md', 'mdx', 'markdown'] }]
  })

  if (typeof selected === 'string') await loadFile(selected)
}

async function openRelative(offset: number) {
  if (currentSiblingIndex.value < 0) return
  const nextPath = siblingFiles.value[currentSiblingIndex.value + offset]
  if (nextPath) await loadFile(nextPath)
}

function toggleTheme() {
  theme.value = theme.value === 'auto' ? 'light' : theme.value === 'light' ? 'dark' : 'auto'
}

async function toggleFullscreen() {
  if (!appWindow) return
  const isFull = await appWindow.isFullscreen()
  await appWindow.setFullscreen(!isFull)
}

async function toggleImmersive() {
  if (!appWindow) {
    showToast('沉浸窗口仅在桌面应用中可用')
    return
  }
  const nextValue = !isImmersive.value
  await appWindow.setDecorations(!nextValue)
  isImmersive.value = nextValue
  localStorage.setItem('markglass-immersive', String(nextValue))
}

async function minimizeWindow() {
  if (!appWindow) return
  await appWindow.minimize()
}

async function toggleMaximize() {
  if (!appWindow) return
  await appWindow.toggleMaximize()
}

async function closeWindow() {
  if (!appWindow) return
  await appWindow.close()
}

function openContextMenu(event: MouseEvent) {
  const width = 244
  const height = 330
  contextMenu.x = Math.max(8, Math.min(event.clientX, window.innerWidth - width - 8))
  contextMenu.y = Math.max(8, Math.min(event.clientY, window.innerHeight - height - 8))
  contextMenu.visible = true
  nextTick(() => contextMenuRef.value?.querySelector<HTMLButtonElement>('button:not(:disabled)')?.focus())
}

function closeContextMenu() {
  contextMenu.visible = false
}

async function runMenuAction(action: () => void | Promise<void>) {
  closeContextMenu()
  await action()
}

async function copySelection() {
  const selectedText = window.getSelection()?.toString() || ''
  if (!selectedText) {
    showToast('请先选择要复制的内容')
    return
  }
  await navigator.clipboard.writeText(selectedText)
  showToast('已复制')
}

function showToast(message: string) {
  toastMessage.value = message
  window.clearTimeout(toastTimer)
  toastTimer = window.setTimeout(() => {
    toastMessage.value = ''
  }, 1800)
}

async function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape' && contextMenu.visible) {
    event.preventDefault()
    closeContextMenu()
    return
  }

  if (event.key === 'F11') {
    event.preventDefault()
    await toggleFullscreen()
    return
  }

  if (event.key === 'Escape') {
    if (!appWindow) return
    const isFull = await appWindow.isFullscreen()
    if (isFull) await appWindow.setFullscreen(false)
    return
  }

  if (event.ctrlKey && event.key.toLowerCase() === 'o') {
    event.preventDefault()
    await openFile()
    return
  }

  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) return

  if (event.key === 'ArrowRight' || event.key === 'PageDown') {
    event.preventDefault()
    await openRelative(1)
  }

  if (event.key === 'ArrowLeft' || event.key === 'PageUp') {
    event.preventDefault()
    await openRelative(-1)
  }
}

function handleSystemThemeChange(event: MediaQueryListEvent) {
  systemIsDark.value = event.matches
}

watch(theme, async (value) => {
  localStorage.setItem('markglass-theme', value)
  if (currentSource.value) await renderDocument(currentSource.value)
})

watch(systemIsDark, async () => {
  if (theme.value === 'auto' && currentSource.value) await renderDocument(currentSource.value)
})

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)
  systemTheme.addEventListener('change', handleSystemThemeChange)

  if (isTauri && appWindow) {
    unlistenOpenFile = await listen<string>('open-file-from-args', async (event) => {
      if (event.payload) await loadFile(event.payload)
    })

    unlistenDragDrop = await appWindow.onDragDropEvent(async (event) => {
      if (event.payload.type === 'enter' || event.payload.type === 'over') {
        isDragging.value = true
        return
      }
      if (event.payload.type === 'leave') {
        isDragging.value = false
        return
      }
      if (event.payload.type === 'drop') {
        isDragging.value = false
        const firstMarkdown = event.payload.paths.find(isMarkdown)
        if (firstMarkdown) await loadFile(firstMarkdown)
        else showToast('拖入的文件不是 Markdown')
      }
    })

    if (isImmersive.value) await appWindow.setDecorations(false)

    const startupFile = await invoke<string | null>('startup_file').catch(() => null)
    if (startupFile) await loadFile(startupFile)
  } else if (new URLSearchParams(window.location.search).has('demo')) {
    currentPath.value = 'MarkGlass 示例.md'
    currentSource.value = demoMarkdown
    siblingFiles.value = [currentPath.value]
    await renderDocument(demoMarkdown)
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown)
  systemTheme.removeEventListener('change', handleSystemThemeChange)
  unlistenOpenFile?.()
  unlistenDragDrop?.()
  window.clearTimeout(toastTimer)
})
</script>
