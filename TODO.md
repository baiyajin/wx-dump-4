# wx-dump-4 项目开发 TODO 列表

## 📋 项目概述

基于 Rust + Vue 3 的微信数据导出工具，支持微信 4.0+ 版本。本TODO列表涵盖了所有需要实现的功能。

---

## 🎯 第一阶段：核心功能完善（优先级：高）

### 1. 后端核心功能完善

#### 1.1 微信信息获取增强
- [ ] **完善文件版本读取**
  - [ ] 修复 `file_version.rs` 中的 Windows API 调用
  - [ ] 从 WeChatWin.dll 读取真实版本信息
  - [ ] 支持从内存映射中获取版本信息（备用方案）
  - [ ] 添加版本缓存机制

- [ ] **完善微信信息获取**
  - [ ] 修复 `wx_info.rs` 中的内存读取逻辑
  - [ ] 实现自动检测版本偏移量（当WX_OFFS.json中没有时）
  - [ ] 支持通过内存搜索获取密钥（备用方案）
  - [ ] 优化多进程支持（微信多开）
  - [ ] 添加错误重试机制

- [ ] **版本偏移量管理**
  - [ ] 实现动态加载偏移量配置
  - [ ] 支持添加新版本偏移量（API接口）
  - [ ] 实现偏移量验证机制
  - [ ] 添加版本兼容性检查

#### 1.2 数据库操作模块

- [ ] **数据库读取模块** (`db/msg.rs`)
  - [ ] 实现 MSG 数据库读取
  - [ ] 实现消息列表查询（分页、筛选）
  - [ ] 实现消息类型解析（文本、图片、视频、语音、文件、表情等）
  - [ ] 实现消息内容解析（XML、Protobuf、LZ4压缩等）
  - [ ] 实现消息时间戳转换
  - [ ] 添加数据库索引优化

- [ ] **联系人数据库** (`db/contact.rs`)
  - [ ] 实现 MicroMsg.db 读取
  - [ ] 获取联系人列表
  - [ ] 获取联系人详细信息（昵称、备注、头像等）
  - [ ] 支持联系人搜索和筛选

- [ ] **媒体数据库** (`db/media.rs`)
  - [ ] 实现 Media.db 读取
  - [ ] 获取媒体文件信息（图片、视频、文件路径）
  - [ ] 实现媒体文件路径解析

- [ ] **其他数据库**
  - [ ] 实现 Favorite.db（收藏）读取
  - [ ] 实现 Sns.db（朋友圈）读取
  - [ ] 实现 PublicMsg.db（公众号消息）读取
  - [ ] 实现 OpenIMContact.db 读取
  - [ ] 实现 OpenIMMedia.db 读取

- [ ] **数据库合并功能** (`db/merge.rs`)
  - [ ] 实现多个MSG数据库合并
  - [ ] 实现数据库去重
  - [ ] 支持时间范围合并
  - [ ] 优化合并性能

#### 1.3 API 接口实现

- [ ] **微信信息API** (`api/wx_info.rs`) ✅ 部分完成
  - [x] `POST /api/wx/info` - 获取微信信息
  - [ ] `GET /api/wx/info/:pid` - 获取指定进程信息
  - [ ] `POST /api/wx/version/offs` - 添加版本偏移量
  - [ ] `GET /api/wx/version/list` - 获取支持的版本列表

- [ ] **数据库API** (`api/db.rs`)
  - [ ] `POST /api/db/init` - 初始化数据库连接
  - [ ] `GET /api/db/status` - 获取数据库状态
  - [ ] `POST /api/db/merge` - 合并数据库
  - [ ] `GET /api/db/list` - 获取数据库列表

- [ ] **聊天记录API** (`api/chat.rs`)
  - [ ] `GET /api/chat/contacts` - 获取联系人列表
  - [ ] `GET /api/chat/contacts/:wxid` - 获取联系人详情
  - [ ] `GET /api/chat/msg/count` - 获取消息数量
  - [ ] `GET /api/chat/msg/count/:wxid` - 获取指定联系人消息数量
  - [ ] `POST /api/chat/msg/list` - 获取消息列表（分页）
  - [ ] `GET /api/chat/msg/:id` - 获取单条消息详情
  - [ ] `POST /api/chat/msg/search` - 搜索消息

- [ ] **媒体文件API** (`api/media.rs`)
  - [ ] `GET /api/media/img/:msg_id` - 获取图片
  - [ ] `GET /api/media/video/:msg_id` - 获取视频
  - [ ] `GET /api/media/audio/:msg_id` - 获取语音
  - [ ] `GET /api/media/file/:msg_id` - 获取文件
  - [ ] `GET /api/media/list/:wxid` - 获取联系人媒体文件列表

- [ ] **导出API** (`api/export.rs`)
  - [ ] `POST /api/export/csv` - 导出CSV
  - [ ] `POST /api/export/json` - 导出JSON
  - [ ] `POST /api/export/html` - 导出HTML
  - [ ] `POST /api/export/pdf` - 导出PDF（可选）
  - [ ] `POST /api/export/docx` - 导出DOCX（可选）
  - [ ] `POST /api/export/dedb` - 导出解密数据库
  - [ ] `POST /api/export/endb` - 导出加密数据库

- [ ] **统计分析API** (`api/statistics.rs`)
  - [ ] `GET /api/stat/contact/:wxid` - 联系人统计
  - [ ] `GET /api/stat/date/chat` - 日期聊天统计
  - [ ] `GET /api/stat/date/heatmap` - 聊天热力图数据
  - [ ] `GET /api/stat/wordcloud/:wxid` - 词云数据

- [ ] **工具API** (`api/tools.rs`)
  - [ ] `POST /api/tools/bias` - 获取偏移量
  - [ ] `POST /api/tools/decrypt` - 解密数据库 ✅ 已实现
  - [ ] `POST /api/tools/merge` - 合并数据库
  - [ ] `GET /api/tools/wxinfo` - 获取微信信息（工具页面）

### 2. 前端页面实现

#### 2.1 首页/仪表板 (`views/Home.vue`) ✅ 部分完成
- [x] 微信信息展示
- [x] 获取微信信息按钮
- [ ] 数据库初始化状态显示
- [ ] 快速操作入口
- [ ] 最近查看的联系人
- [ ] 统计概览卡片

#### 2.2 聊天记录页面 (`views/Chat.vue`)
- [ ] **联系人列表组件** (`components/chat/ContactsList.vue`)
  - [ ] 联系人列表展示
  - [ ] 联系人搜索功能
  - [ ] 联系人筛选（好友/群聊/公众号）
  - [ ] 联系人排序（按消息数量/时间）
  - [ ] 联系人头像显示
  - [ ] 消息数量显示

- [ ] **聊天记录主组件** (`components/chat/ChatRecordsMain.vue`)
  - [ ] 消息列表展示
  - [ ] 无限滚动加载（向上/向下）
  - [ ] 消息类型渲染（文本、图片、视频、语音、文件、表情等）
  - [ ] 消息时间显示
  - [ ] 发送者信息显示
  - [ ] 消息搜索高亮
  - [ ] 消息跳转功能

- [ ] **消息类型组件**
  - [ ] `MessageText.vue` - 文本消息
  - [ ] `MessageImg.vue` - 图片消息（支持预览）
  - [ ] `MessageVideo.vue` - 视频消息（支持播放）
  - [ ] `MessageAudio.vue` - 语音消息（支持播放）
  - [ ] `MessageFile.vue` - 文件消息（支持下载）
  - [ ] `MessageEmoji.vue` - 表情消息
  - [ ] `MessageOther.vue` - 其他类型消息

- [ ] **聊天记录头部** (`components/chat/ChatRecordsHeader.vue`)
  - [ ] 联系人信息展示
  - [ ] 消息统计信息
  - [ ] 操作按钮（导出、搜索等）

#### 2.3 统计分析页面 (`views/Statistics.vue`)
- [ ] **聊天热力图** (`components/stats/DateChatHeatmapStats.vue`)
  - [ ] 日历热力图展示
  - [ ] 按日期显示聊天数量
  - [ ] 支持筛选联系人
  - [ ] 支持时间范围选择

- [ ] **日期聊天统计** (`components/stats/DateChatStats.vue`)
  - [ ] 折线图展示每日聊天数量
  - [ ] 支持多联系人对比
  - [ ] 支持时间范围选择
  - [ ] 数据导出功能

- [ ] **联系人画像** (`components/stats/ContactStats.vue`)
  - [ ] 联系人聊天统计
  - [ ] 消息类型分布（饼图）
  - [ ] 聊天时间分布
  - [ ] 词云生成（可选）

#### 2.4 数据导出页面 (`views/Export.vue`)
- [ ] **导出主组件** (`components/chatBackup/ChatExportMain.vue`)
  - [ ] 导出类型选择
  - [ ] 导出参数配置
  - [ ] 导出进度显示
  - [ ] 导出历史记录

- [ ] **导出格式组件**
  - [ ] `ExportCSV.vue` - CSV导出
  - [ ] `ExportJSON.vue` - JSON导出
  - [ ] `ExportHTML.vue` - HTML导出
  - [ ] `ExportPDF.vue` - PDF导出（可选）
  - [ ] `ExportDOCX.vue` - DOCX导出（可选）
  - [ ] `ExportDEDB.vue` - 解密数据库导出
  - [ ] `ExportENDB.vue` - 加密数据库导出

#### 2.5 其他页面

- [ ] **联系人页面** (`views/Contacts.vue`)
  - [ ] 联系人列表
  - [ ] 联系人详情
  - [ ] 联系人搜索
  - [ ] 联系人分组

- [ ] **收藏页面** (`views/Favorite.vue`)
  - [ ] 收藏列表展示
  - [ ] 收藏内容预览
  - [ ] 收藏搜索

- [ ] **朋友圈页面** (`views/Moments.vue`)
  - [ ] 朋友圈内容展示
  - [ ] 朋友圈时间线
  - [ ] 朋友圈搜索

- [ ] **清理工具页面** (`views/Cleanup.vue`)
  - [ ] 存储空间分析
  - [ ] 媒体文件清理
  - [ ] 清理进度显示

- [ ] **设置页面** (`views/Settings.vue`)
  - [ ] 数据库路径配置
  - [ ] 导出路径配置
  - [ ] 主题设置
  - [ ] 语言设置

- [ ] **工具页面** (`views/Tools.vue`)
  - [ ] 偏移量获取工具
  - [ ] 数据库解密工具
  - [ ] 数据库合并工具
  - [ ] 微信信息获取工具

---

## 🔧 第二阶段：功能增强（优先级：中）

### 3. 后端功能增强

#### 3.1 性能优化
- [ ] 实现数据库连接池
- [ ] 实现查询结果缓存
- [ ] 优化内存搜索算法
- [ ] 实现异步数据库操作
- [ ] 添加批量操作支持

#### 3.2 错误处理
- [ ] 完善错误类型定义
- [ ] 添加详细错误日志
- [ ] 实现错误恢复机制
- [ ] 添加用户友好的错误提示

#### 3.3 安全性
- [ ] 添加API认证（可选）
- [ ] 实现请求限流
- [ ] 添加数据加密传输（HTTPS）
- [ ] 实现敏感信息脱敏

### 4. 前端功能增强

#### 4.1 UI/UX 优化
- [ ] 实现响应式设计
- [ ] 添加暗色主题支持
- [ ] 优化加载动画
- [ ] 添加骨架屏
- [ ] 实现消息虚拟滚动（性能优化）

#### 4.2 交互功能
- [ ] 实现消息搜索高亮
- [ ] 实现消息跳转
- [ ] 实现消息复制
- [ ] 实现消息转发（导出）
- [ ] 实现批量操作

#### 4.3 数据可视化
- [ ] 集成图表库（ECharts/Chart.js）
- [ ] 实现词云生成
- [ ] 实现聊天热力图
- [ ] 实现统计图表

---

## 🚀 第三阶段：高级功能（优先级：低）

### 5. 高级功能

- [ ] **实时消息监控**
  - [ ] 实现实时消息推送（WebSocket）
  - [ ] 消息通知功能

- [ ] **批量导出**
  - [ ] 批量导出多个联系人
  - [ ] 导出任务队列
  - [ ] 导出进度跟踪

- [ ] **数据分析**
  - [ ] 聊天情感分析
  - [ ] 聊天关键词提取
  - [ ] 聊天趋势分析

- [ ] **备份恢复**
  - [ ] 数据备份功能
  - [ ] 数据恢复功能
  - [ ] 备份文件管理

---

## 🐛 第四阶段：测试和优化（持续进行）

### 6. 测试

- [ ] **单元测试**
  - [ ] 后端核心模块测试
  - [ ] 数据库操作测试
  - [ ] API接口测试

- [ ] **集成测试**
  - [ ] 前后端集成测试
  - [ ] 端到端测试

- [ ] **性能测试**
  - [ ] 数据库查询性能
  - [ ] 内存操作性能
  - [ ] API响应时间

### 7. 文档

- [ ] **开发文档**
  - [ ] API文档（OpenAPI/Swagger）
  - [ ] 代码注释完善
  - [ ] 架构设计文档

- [ ] **用户文档**
  - [ ] 使用指南
  - [ ] 常见问题（FAQ）
  - [ ] 故障排除指南

---

## 📝 技术债务

### 8. 代码质量

- [ ] 添加代码格式化（rustfmt）
- [ ] 添加代码检查（clippy）
- [ ] 添加前端代码检查（ESLint）
- [ ] 统一错误处理模式
- [ ] 优化代码结构

### 9. 依赖管理

- [ ] 更新依赖版本
- [ ] 移除未使用的依赖
- [ ] 优化编译时间
- [ ] 优化打包体积

---

## 🎯 实现优先级说明

- **P0（最高）**：核心功能，必须实现
- **P1（高）**：重要功能，优先实现
- **P2（中）**：增强功能，后续实现
- **P3（低）**：可选功能，有时间再实现

---

## 📊 进度跟踪

### 已完成 ✅
- [x] 项目基础结构
- [x] Rust后端核心框架
- [x] Vue3前端基础框架
- [x] 微信信息获取（基础）
- [x] 数据库解密（基础）
- [x] Git配置和自动提交脚本
- [x] 版本偏移量配置导入

### 进行中 🔄
- [ ] 数据库读取模块
- [ ] 聊天记录API
- [ ] 前端聊天记录页面

### 待开始 ⏳
- [ ] 统计分析功能
- [ ] 数据导出功能
- [ ] 高级功能

---

## 📅 预计时间线

- **第一阶段**：2-3周（核心功能）
- **第二阶段**：1-2周（功能增强）
- **第三阶段**：1-2周（高级功能）
- **第四阶段**：持续进行（测试和优化）

---

## 🔗 相关资源

- 原项目：https://github.com/baiyajin/wx-dump-plus
- PyWxDump：https://github.com/xaoyaoo/PyWxDump
- 项目仓库：https://github.com/baiyajin/wx-dump-4

---

**最后更新**：2024-01-XX
**维护者**：baiyajin

