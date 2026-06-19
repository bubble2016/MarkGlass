# MarkGlass 设计 QA

- source visual truth path: `C:\Users\win8e\AppData\Local\Temp\codex-clipboard-f87d7322-da39-4ec0-86d2-ec274acbd30b.png`
- implementation screenshot path: `D:\md-reader-windows\design-implementation.png`
- combined comparison path: `D:\md-reader-windows\design-comparison.png`
- viewport: `1536 × 964`
- state: 亮色主题、第一篇文档、文件侧栏展开、100% 缩放、目录收起

## Full-view comparison evidence

参考图与实现图已按相同可用窗口尺寸并排比较。实现保持了相同的主要构图：左侧约 250px 玻璃文件栏、右侧大面积白色圆角阅读卡、顶部中央状态胶囊、卡片两侧翻页按钮，以及底部中央深色悬浮工具条。背景壁纸、玻璃透明度、卡片阴影和控件层级均达到同一视觉方向。

## Focused region comparison evidence

- 左侧文件栏：标题、缩略图比例、活动项描边、底部计数和纵向密度与参考图一致。
- 顶部与底部浮层：文件进度胶囊和工具条均为独立毛玻璃浮层，位置已按最后一轮对比校准。
- 阅读内容：标题层级、Markdown 标记、右浮山湖图片、任务列表、表格和 Mermaid 双栏结构与参考图对应。
- 翻页按钮：圆形深色按钮跨在阅读卡边缘，尺寸、阴影和纵向位置一致。

## Required fidelity surfaces

- Fonts and typography: 使用 Segoe UI Variable/Text 与微软雅黑回退；标题字重、正文行高、小号控件文字和截断均匹配 Windows 11 阅读界面。
- Spacing and layout rhythm: 外边距、卡片圆角、侧栏宽度、内容内边距、悬浮层间距和阴影层级已核对。
- Colors and visual tokens: 白色阅读面、深蓝灰玻璃、蓝色强调和绿色任务状态与参考图一致；暗色主题也有对应令牌。
- Image quality and asset fidelity: 使用从用户参考图提取的山湖配图和真实文档缩略图，没有可见占位资源。
- Copy and content: 文件名、进度、目录文件、核心特性、表格和 Mermaid 示例均与目标语义对应。

## Findings

无可执行的 P0、P1 或 P2 差异。

## Patches made

- 将窗口改为透明、无边框、启动最大化且保留系统任务栏。
- 接入 Windows Acrylic 原生背景效果。
- 重建侧栏、阅读卡、进度胶囊、翻页按钮和底部工具条。
- 增加目录、缩放、主题、字体、打开文件、全屏和侧栏切换交互。
- 增加从左下文件位置展开及反向缩回的开关动画。
- 根据并排比较调整示例内容双栏、标题标记、图片位置和浮层中心点。

## Follow-up polish

- 浏览器预览用固定壁纸模拟桌面；Tauri 桌面版会显示用户实际桌面并由 Acrylic 处理。
- 示例预览使用 8 个文件，真实应用会按当前目录实际文件数量显示。

final result: passed
