<template>
  <main
    class="app-shell"
    :class="[
      themeClass,
      {
        'is-dragging': isDragging,
        'is-closing': isClosing,
        'sidebar-collapsed': !sidebarVisible,
        'is-serif': serifMode,
        'web-preview': !isTauri
      }
    ]"
    @click="closeContextMenu"
    @contextmenu.prevent="openContextMenu"
  >
    <div class="desktop-tint" aria-hidden="true"></div>

    <section v-if="isBooting" class="boot-stage" aria-live="polite" aria-label="MarkGlass 正在启动">
      <img :src="markGlassWordmarkDark" alt="MarkGlass" />
      <span>像浏览照片一样阅读 Markdown</span>
    </section>

    <section v-if="isLoading && !html" class="empty-stage status-state" aria-live="polite">
      <LoaderCircle class="spin" :size="34" />
      <p>正在打开 Markdown…</p>
    </section>

    <section v-else-if="errorMessage && !html" class="empty-stage status-state" role="alert">
      <AlertCircle :size="38" />
      <h1>无法打开文件</h1>
      <p>{{ errorMessage }}</p>
      <button class="primary-button" type="button" @click="openFile">
        <FolderOpen :size="18" />
        选择其他文件
      </button>
    </section>

    <section v-else-if="!html" class="empty-stage">
      <div class="welcome-card">
        <picture class="brand-lockup">
          <source media="(prefers-color-scheme: dark)" :srcset="markGlassWordmarkDark" />
          <img
            :src="effectiveDark ? markGlassWordmarkDark : markGlassWordmarkLight"
            alt="MarkGlass"
          />
        </picture>
        <p class="eyebrow">IMMERSIVE MARKDOWN READER</p>
        <p>把 Markdown 像照片一样铺展在桌面上。</p>
        <button class="primary-button" type="button" @click="openFile">
          <FolderOpen :size="18" />
          打开 Markdown
        </button>
        <div v-if="recentFiles.length" class="recent-files-list">
          <button v-for="file in recentFiles" :key="file.path" type="button" @click="loadFile(file.path)">
            <FileText :size="17" />
            <span>{{ file.name }}</span>
          </button>
        </div>
        <div class="hint">
          <span><kbd>Ctrl</kbd> + <kbd>O</kbd> 打开</span>
          <span><kbd>←</kbd> <kbd>→</kbd> 连续阅读</span>
        </div>
      </div>
    </section>

    <div v-else class="immersive-stage">
      <aside v-if="sidebarVisible && siblingFiles.length > 1" class="file-rail" aria-label="同目录文件">
        <div class="rail-header">
          <strong>同目录文件</strong>
          <button type="button" aria-label="收起文件列表" title="收起文件列表" @click.stop="sidebarVisible = false">
            <ChevronLeft :size="19" />
          </button>
        </div>

        <div class="rail-list">
          <button
            v-for="(file, index) in siblingFiles"
            :key="file"
            class="file-card"
            :class="{ 'is-active': samePath(file, currentPath) }"
            type="button"
            @click.stop="loadFile(file)"
          >
            <img :src="documentThumbnail" alt="" />
            <span class="file-copy">
              <strong>{{ fileName(file) }}</strong>
              <small>{{ samePath(file, currentPath) ? '正在阅读' : index + 1 }}</small>
            </span>
          </button>
        </div>

        <div class="rail-footer">
          <span>共 {{ siblingFiles.length }} 个文件</span>
          <Check :size="16" />
        </div>
      </aside>

      <button
        v-if="!sidebarVisible && siblingFiles.length > 1"
        class="rail-reveal glass-button"
        type="button"
        aria-label="显示同目录文件"
        title="显示同目录文件"
        @click.stop="sidebarVisible = true"
      >
        <PanelLeftOpen :size="20" />
      </button>

      <div class="document-status glass-pill" data-tauri-drag-region>
        <strong data-tauri-drag-region>{{ currentFileName }}</strong>
        <span data-tauri-drag-region></span>
        <span data-tauri-drag-region>{{ displayIndex }} / {{ siblingFiles.length || 1 }}</span>
      </div>

      <div class="window-controls">
        <button type="button" aria-label="最小化窗口" title="最小化" @click.stop="minimizeWindow">
          <Minus :size="16" />
        </button>
        <button type="button" aria-label="关闭窗口" title="关闭" @click.stop="closeWindow">
          <X :size="17" />
        </button>
      </div>

      <section
        ref="readerWrapRef"
        class="reader-card"
        :aria-busy="isLoading"
        :style="{ '--reader-zoom': `${zoom / 100}` }"
        @scroll="handleScroll"
      >
        <article
          ref="articleRef"
          class="markdown-body"
          :class="{ 'is-demo-document': isDemoDocument }"
          v-html="html"
        ></article>

        <div v-if="errorMessage" class="inline-error" role="alert">
          <AlertCircle :size="17" />
          <span>{{ errorMessage }}</span>
          <button type="button" aria-label="关闭错误提示" @click="errorMessage = ''"><X :size="15" /></button>
        </div>
      </section>

      <button
        class="page-nav page-nav-prev"
        type="button"
        aria-label="上一篇"
        title="上一篇（←）"
        :disabled="!canOpenPrevious"
        @click.stop="openRelative(-1)"
      >
        <ChevronLeft :size="30" />
      </button>
      <button
        class="page-nav page-nav-next"
        type="button"
        aria-label="下一篇"
        title="下一篇（→）"
        :disabled="!canOpenNext"
        @click.stop="openRelative(1)"
      >
        <ChevronRight :size="30" />
      </button>

      <aside
        v-if="toc.length"
        class="toc-dock"
        :class="{ 'is-expanded': tocVisible }"
        aria-label="文档目录"
        @click.stop
      >
        <div v-if="tocVisible" class="toc-panel">
          <div class="toc-panel-header">
            <strong>文档目录</strong>
            <span>{{ toc.length }} 项</span>
          </div>
          <nav>
            <button
              v-for="item in toc"
              :key="item.id"
              type="button"
              :class="[`toc-level-${item.level}`, { 'is-active': activeTocId === item.id }]"
              @click="scrollToHeading(item.id)"
            >
              {{ item.text }}
            </button>
          </nav>
        </div>
        <button
          class="toc-dock-toggle"
          type="button"
          :aria-expanded="tocVisible"
          :aria-label="tocVisible ? '收起文档目录' : '展开文档目录'"
          :title="tocVisible ? '收起文档目录' : '展开文档目录'"
          @click="tocVisible = !tocVisible"
        >
          <ListTree :size="20" />
          <span v-if="tocVisible">收起目录</span>
        </button>
      </aside>

      <nav class="floating-toolbar" aria-label="阅读工具栏">
        <button
          type="button"
          :class="{ 'is-active': sidebarVisible }"
          :aria-expanded="sidebarVisible"
          aria-label="同目录文件"
          title="同目录文件"
          @click.stop="sidebarVisible = !sidebarVisible"
        >
          <LayoutGrid :size="19" />
        </button>
        <span class="toolbar-separator"></span>
        <button type="button" aria-label="缩小" title="缩小" :disabled="zoom <= 70" @click.stop="setZoom(zoom - 10)">
          <ZoomOut :size="19" />
        </button>
        <button class="zoom-value" type="button" title="恢复 100%" @click.stop="setZoom(100)">{{ zoom }}%</button>
        <button type="button" aria-label="放大" title="放大" :disabled="zoom >= 160" @click.stop="setZoom(zoom + 10)">
          <ZoomIn :size="19" />
        </button>
        <span class="toolbar-separator"></span>
        <button type="button" :aria-label="`当前主题：${themeLabel}`" :title="`主题：${themeLabel}`" @click.stop="toggleTheme">
          <Sun v-if="!effectiveDark" :size="20" />
          <Moon v-else :size="19" />
        </button>
        <button
          type="button"
          :class="{ 'is-active': serifMode }"
          aria-label="切换阅读字体"
          title="切换阅读字体"
          @click.stop="serifMode = !serifMode"
        >
          <Type :size="19" />
        </button>
        <button type="button" aria-label="打开文件" title="打开文件（Ctrl+O）" @click.stop="openFile">
          <FolderOpen :size="19" />
        </button>
        <button type="button" aria-label="切换全屏" title="全屏（F11）" @click.stop="toggleFullscreen">
          <Maximize2 :size="19" />
        </button>
      </nav>
    </div>

    <div
      v-if="contextMenu.visible"
      ref="contextMenuRef"
      class="context-menu"
      role="menu"
      :style="{ left: `${contextMenu.x}px`, top: `${contextMenu.y}px` }"
      @click.stop
    >
      <button type="button" role="menuitem" @click="runMenuAction(copySelection)">
        <Copy :size="16" /><span>复制所选内容</span><kbd>Ctrl+C</kbd>
      </button>
      <button type="button" role="menuitem" @click="runMenuAction(openFile)">
        <FolderOpen :size="16" /><span>打开文件</span><kbd>Ctrl+O</kbd>
      </button>
      <div class="menu-separator"></div>
      <button type="button" role="menuitem" :disabled="!canOpenPrevious" @click="runMenuAction(() => openRelative(-1))">
        <ChevronLeft :size="16" /><span>上一篇</span><kbd>←</kbd>
      </button>
      <button type="button" role="menuitem" :disabled="!canOpenNext" @click="runMenuAction(() => openRelative(1))">
        <ChevronRight :size="16" /><span>下一篇</span><kbd>→</kbd>
      </button>
      <div class="menu-separator"></div>
      <button type="button" role="menuitem" @click="runMenuAction(toggleTheme)">
        <Palette :size="16" /><span>切换主题</span><small>{{ themeLabel }}</small>
      </button>
      <button type="button" role="menuitem" @click="runMenuAction(exportPdf)">
        <Printer :size="16" /><span>导出 PDF</span>
      </button>
      <button type="button" role="menuitem" @click="runMenuAction(toggleFullscreen)">
        <Maximize2 :size="16" /><span>全屏</span><kbd>F11</kbd>
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
  FileText,
  FolderOpen,
  LayoutGrid,
  ListTree,
  LoaderCircle,
  Maximize2,
  Minus,
  Moon,
  Palette,
  PanelLeftOpen,
  Printer,
  Sun,
  Type,
  X,
  ZoomIn,
  ZoomOut
} from '@lucide/vue'
import markGlassWordmarkLight from './assets/markglass-wordmark-light.png'
import markGlassWordmarkDark from './assets/markglass-wordmark-dark.png'
import documentThumbnail from './assets/document-thumbnail.png'
import demoLandscape from './assets/demo-landscape.png'

interface MarkdownDocument {
  path: string
  content: string
  siblings: string[]
}

interface TocItem {
  id: string
  text: string
  level: number
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
const isBooting = ref(true)
const isClosing = ref(false)
const errorMessage = ref('')
const toastMessage = ref('')
const articleRef = ref<HTMLElement | null>(null)
const readerWrapRef = ref<HTMLElement | null>(null)
const contextMenuRef = ref<HTMLElement | null>(null)
const sidebarVisible = ref(false)
const tocVisible = ref(false)
const serifMode = ref(localStorage.getItem('markglass-serif') === 'true')
const zoom = ref(Number(localStorage.getItem('markglass-zoom')) || 100)
const theme = ref<Theme>((localStorage.getItem('markglass-theme') as Theme) || 'auto')
const systemTheme = window.matchMedia('(prefers-color-scheme: dark)')
const systemIsDark = ref(systemTheme.matches)
const contextMenu = reactive({ visible: false, x: 0, y: 0 })
const toc = ref<TocItem[]>([])
const activeTocId = ref('')
const recentFiles = ref<{ name: string; path: string }[]>([])

try {
  recentFiles.value = JSON.parse(localStorage.getItem('markglass-recent-files') || '[]')
} catch {}

const demoNames = [
  '01. README.md',
  '02. 快速开始.md',
  '03. 功能特性.md',
  '04. 使用指南.md',
  '05. 主题定制.md',
  '06. 快捷键.md',
  '07. 常见问题.md',
  '08. 更新日志.md'
]

const demoMarkdown = `# MarkGlass

> 像看图片一样阅读 Markdown

<img class="demo-hero" src="${demoLandscape}" alt="山湖风景">

MarkGlass 是一款专为 Markdown 阅读设计的 Windows 桌面应用，

追求极简、美观、流畅的阅读体验。

## ✨ 核心特性

- [x] 秒开极速
- [x] 沉浸阅读
- [x] 完美渲染
- [x] 连续阅读
- [x] 主题切换
- [x] 代码高亮

## 📊 表格示例

| 特性 | 说明 | 支持 |
| --- | --- | :---: |
| 代码高亮 | 支持多种语言代码高亮 | ✓ |
| Mermaid | 支持流程图、时序图等 | ✓ |
| 任务列表 | 支持任务列表和复选框 | ✓ |
| 表格渲染 | 支持复杂表格样式 | ✓ |

## 📈 Mermaid 示例

\`\`\`mermaid
flowchart TD
  A[阅读 Markdown] --> B[渲染内容]
  B --> C{是否有链接?}
  C -- 否 --> D[显示内容]
  C -- 是 --> E[显示链接目标]
\`\`\`
`

let toastTimer: number | undefined
let scrollTimeout: number | undefined
let renderSequence = 0
let tocObserver: IntersectionObserver | null = null
let unlistenOpenFile: UnlistenFn | undefined
let unlistenDragDrop: UnlistenFn | undefined

const themeClass = computed(() => `theme-${theme.value}`)
const themeLabel = computed(() => (theme.value === 'auto' ? '跟随系统' : theme.value === 'light' ? '亮色' : '暗色'))
const currentFileName = computed(() => fileName(currentPath.value))
const currentSiblingIndex = computed(() =>
  siblingFiles.value.findIndex((path) => samePath(path, currentPath.value))
)
const displayIndex = computed(() => Math.max(1, currentSiblingIndex.value + 1))
const canOpenPrevious = computed(() => currentSiblingIndex.value > 0)
const canOpenNext = computed(() =>
  currentSiblingIndex.value >= 0 && currentSiblingIndex.value < siblingFiles.value.length - 1
)
const effectiveDark = computed(() => theme.value === 'dark' || (theme.value === 'auto' && systemIsDark.value))
const isDemoDocument = computed(() => !isTauri && demoNames.includes(currentPath.value))

const md = new MarkdownIt({ html: true, linkify: true, typographer: true })
  .use(anchor)
  .use(footnote)
  .use(taskLists, { enabled: true, label: true })

const highlighterPromise = createHighlighter({
  themes: ['github-light', 'github-dark'],
  langs: ['bash', 'c', 'cpp', 'css', 'diff', 'html', 'java', 'javascript', 'json', 'jsx', 'markdown', 'powershell', 'python', 'rust', 'sql', 'typescript', 'tsx', 'vue', 'yaml']
})

function fileName(path: string) {
  return path.split(/[\\/]/).pop() || 'MarkGlass'
}

function samePath(a: string, b: string) {
  return a.toLowerCase() === b.toLowerCase()
}

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

function isMarkdown(path: string) {
  return ['md', 'mdx', 'markdown'].includes(path.split('.').pop()?.toLowerCase() || '')
}

function normalizeLanguage(language: string) {
  const aliases: Record<string, string> = {
    js: 'javascript', ts: 'typescript', shell: 'bash', sh: 'bash', ps1: 'powershell',
    py: 'python', rs: 'rust', yml: 'yaml', md: 'markdown', text: 'plaintext', txt: 'plaintext'
  }
  return aliases[language] || language || 'plaintext'
}

async function renderCodeBlocks(sequence: number) {
  if (!articleRef.value) return
  const highlighter = await highlighterPromise
  if (sequence !== renderSequence) return
  const blocks = Array.from(articleRef.value.querySelectorAll('pre > code')).filter(
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
    const wrapper = document.createElement('div')
    wrapper.className = 'code-block'
    wrapper.innerHTML = highlighted
    code.parentElement?.replaceWith(wrapper)
  }
}

async function renderMermaid(sequence: number) {
  if (!articleRef.value) return
  mermaid.initialize({
    startOnLoad: false,
    securityLevel: 'strict',
    theme: effectiveDark.value ? 'dark' : 'default',
    fontFamily: '"Segoe UI", "Microsoft YaHei", sans-serif'
  })
  let index = 0
  for (const code of Array.from(articleRef.value.querySelectorAll('code.language-mermaid'))) {
    const container = document.createElement('div')
    container.className = 'mermaid-render'
    try {
      const { svg } = await mermaid.render(`markglass-mermaid-${sequence}-${index++}`, code.textContent || '')
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
  extractToc()
  restoreScrollProgress()
  await Promise.all([renderCodeBlocks(sequence), renderMermaid(sequence)])
  if (sequence === renderSequence) {
    extractToc()
  }
}

function extractToc() {
  if (!articleRef.value) return
  const headings = Array.from(articleRef.value.querySelectorAll('h1, h2, h3, h4'))
  toc.value = headings.map((heading, index) => {
    if (!heading.id) heading.id = `heading-${index}`
    return { id: heading.id, text: heading.textContent || '', level: Number(heading.tagName[1]) }
  })
  tocObserver?.disconnect()
  tocObserver = new IntersectionObserver((entries) => {
    const active = entries.find((entry) => entry.isIntersecting)
    if (active) activeTocId.value = active.target.id
  }, { root: readerWrapRef.value, rootMargin: '-8% 0px -78% 0px' })
  headings.forEach((heading) => tocObserver?.observe(heading))
}

function scrollToHeading(id: string) {
  const heading = document.getElementById(id)
  if (heading && readerWrapRef.value) {
    readerWrapRef.value.scrollTo({ top: heading.offsetTop - 70, behavior: 'smooth' })
  }
}

function handleScroll() {
  window.clearTimeout(scrollTimeout)
  scrollTimeout = window.setTimeout(saveScrollProgress, 250)
}

function saveScrollProgress() {
  if (currentPath.value && readerWrapRef.value) {
    localStorage.setItem(`markglass-scroll-${currentPath.value}`, String(readerWrapRef.value.scrollTop))
  }
}

function restoreScrollProgress() {
  if (!readerWrapRef.value) return
  const saved = localStorage.getItem(`markglass-scroll-${currentPath.value}`)
  readerWrapRef.value.scrollTo({ top: saved ? Number(saved) : 0, behavior: 'instant' })
}

function addToRecent(path: string, name: string) {
  recentFiles.value = [{ name, path }, ...recentFiles.value.filter((file) => file.path !== path)].slice(0, 8)
  localStorage.setItem('markglass-recent-files', JSON.stringify(recentFiles.value))
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
    if (!isTauri) {
      currentPath.value = normalizedPath
      currentSource.value = demoMarkdown
      siblingFiles.value = demoNames
      await renderDocument(demoMarkdown)
      return
    }
    const document = await invoke<MarkdownDocument>('open_markdown', { path: normalizedPath })
    currentPath.value = document.path
    currentSource.value = document.content
    siblingFiles.value = document.siblings
    addToRecent(document.path, fileName(document.path))
    await renderDocument(document.content)
    window.document.title = `${currentFileName.value} — MarkGlass`
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : String(error)
  } finally {
    isLoading.value = false
  }
}

async function openFile() {
  if (!isTauri) {
    showToast('桌面应用中可打开本地 Markdown')
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
  const nextPath = siblingFiles.value[currentSiblingIndex.value + offset]
  if (nextPath) await loadFile(nextPath)
}

function setZoom(value: number) {
  zoom.value = Math.min(160, Math.max(70, value))
}

function exportPdf() {
  window.print()
}

function toggleTheme() {
  theme.value = effectiveDark.value ? 'light' : 'dark'
}

async function toggleFullscreen() {
  if (!appWindow) {
    showToast('全屏切换仅在桌面应用中可用')
    return
  }
  await appWindow.setFullscreen(!(await appWindow.isFullscreen()))
}

async function minimizeWindow() {
  await appWindow?.minimize()
}

async function closeWindow() {
  if (isClosing.value) return
  isClosing.value = true
  window.setTimeout(async () => {
    if (appWindow) await appWindow.close()
    else isClosing.value = false
  }, 240)
}

function openContextMenu(event: MouseEvent) {
  contextMenu.x = Math.max(8, Math.min(event.clientX, window.innerWidth - 244))
  contextMenu.y = Math.max(8, Math.min(event.clientY, window.innerHeight - 320))
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
  if (!selectedText) return showToast('请先选择要复制的内容')
  await navigator.clipboard.writeText(selectedText)
  showToast('已复制')
}

function showToast(message: string) {
  toastMessage.value = message
  window.clearTimeout(toastTimer)
  toastTimer = window.setTimeout(() => (toastMessage.value = ''), 1800)
}

async function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    event.preventDefault()
    return closeWindow()
  }
  if (event.key === 'F11') {
    event.preventDefault()
    return toggleFullscreen()
  }
  if (event.ctrlKey && event.key.toLowerCase() === 'o') {
    event.preventDefault()
    return openFile()
  }
  if (event.ctrlKey && (event.key === '+' || event.key === '=')) {
    event.preventDefault()
    return setZoom(zoom.value + 10)
  }
  if (event.ctrlKey && event.key === '-') {
    event.preventDefault()
    return setZoom(zoom.value - 10)
  }
  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) return
  if (event.key === 'ArrowRight' || event.key === 'PageDown') {
    event.preventDefault()
    return openRelative(1)
  }
  if (event.key === 'ArrowLeft' || event.key === 'PageUp') {
    event.preventDefault()
    return openRelative(-1)
  }
}

watch(theme, async (value) => {
  localStorage.setItem('markglass-theme', value)
  if (currentSource.value) await renderDocument(currentSource.value)
})
watch(systemIsDark, async () => {
  if (theme.value === 'auto' && currentSource.value) await renderDocument(currentSource.value)
})
watch(serifMode, (value) => localStorage.setItem('markglass-serif', String(value)))
watch(zoom, (value) => localStorage.setItem('markglass-zoom', String(value)))

onMounted(async () => {
  const bootStartedAt = performance.now()
  window.addEventListener('keydown', handleKeydown)
  systemTheme.addEventListener('change', (event) => (systemIsDark.value = event.matches))

  try {
    if (isTauri && appWindow) {
      unlistenOpenFile = await listen<string>('open-file-from-args', async (event) => {
        if (event.payload) await loadFile(event.payload)
      })
      unlistenDragDrop = await appWindow.onDragDropEvent(async (event) => {
        if (event.payload.type === 'enter' || event.payload.type === 'over') return void (isDragging.value = true)
        if (event.payload.type === 'leave') return void (isDragging.value = false)
        if (event.payload.type === 'drop') {
          isDragging.value = false
          const firstMarkdown = event.payload.paths.find(isMarkdown)
          if (firstMarkdown) await loadFile(firstMarkdown)
          else showToast('拖入的文件不是 Markdown')
        }
      })
      const startupFile = await invoke<string | null>('startup_file').catch(() => null)
      if (startupFile) await loadFile(startupFile)
    } else if (new URLSearchParams(window.location.search).has('demo')) {
      await loadFile(demoNames[0])
    }
  } finally {
    const remaining = Math.max(0, 360 - (performance.now() - bootStartedAt))
    if (remaining) await new Promise((resolve) => window.setTimeout(resolve, remaining))
    isBooting.value = false
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown)
  unlistenOpenFile?.()
  unlistenDragDrop?.()
  tocObserver?.disconnect()
  window.clearTimeout(toastTimer)
  window.clearTimeout(scrollTimeout)
})
</script>
