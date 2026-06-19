# MarkGlass

MarkGlass 是一个轻量、沉浸式的 Windows Markdown 阅读器，让 `.md` 文件像图片一样双击即读。

## 功能

- 双击 `.md`、`.mdx`、`.markdown` 文件直接打开
- 已运行时再次双击文件会复用现有窗口
- 拖拽打开 Markdown
- Shiki 代码高亮与 Mermaid 图表
- 亮色、暗色、跟随系统主题
- 同目录 Markdown 连续阅读
- 自定义右键菜单
- F11 全屏与无边框沉浸窗口
- Windows 文件关联及 MSI、NSIS 安装包

## 开发

```bash
npm install
npm run desktop
```

仅运行前端页面：

```bash
npm run dev
```

## 构建

```bash
npm run bundle
```
