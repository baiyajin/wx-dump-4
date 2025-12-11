# wx-dump-4

<div align="center">
  <img src="logo.png" alt="wx-dump-4 Logo" width="200">
</div>

基于 Rust + Vue 3 的微信数据导出工具，支持微信 4.0+ 版本。

## 功能特性

<details>
<summary>点击展开查看完整功能列表</summary>

### 已实现功能 ✅

#### 核心功能
- ✅ 进程管理（进程枚举、进程句柄管理、查找 WeChat.exe 进程）
- ✅ 内存操作（内存读取、内存搜索、内存映射查询、字符串和指针读取）
- ✅ 版本适配（版本号解析、地址长度检测、偏移量管理）
- ✅ 微信信息获取（获取微信账号、昵称、手机号、邮箱、密钥、目录路径）
- ✅ 数据库解密（SQLite 数据库解密、支持批量解密、AES-256-CBC 解密算法）
- ✅ 文件版本信息读取（获取微信版本信息）
- ✅ 多进程支持（支持微信多开场景）
- ✅ **版本偏移量自动检测**（通过内存搜索特征码自动定位偏移量，无需手动配置）
- ✅ **Protobuf解析器**（解析微信消息中的BytesExtra字段，优先使用Protobuf解析）
- ✅ **数据库查询优化**（索引优化、缓存机制，提升查询性能）
- ✅ **错误处理增强**（扩展错误类型，提供用户友好的错误消息）
- ✅ **聊天记录读取**（从解密后的数据库读取聊天记录）
- ✅ **聊天记录搜索**（支持关键词搜索）
- ✅ **聊天记录筛选**（按时间、联系人、消息类型筛选）
- ✅ **数据导出**（支持CSV、JSON、HTML格式导出）
- ✅ **统计分析**（聊天数据统计、可视化图表、活跃度分析、联系人分析）

#### API 接口
- ✅ `POST /api/wx/info` - 获取微信信息
- ✅ `GET /api/wx/info/:pid` - 根据进程ID获取微信信息
- ✅ `GET /api/wx/version/list` - 获取支持的版本列表
- ✅ `POST /api/wx/version/offs` - 添加版本偏移量配置
- ✅ `POST /api/wx/decrypt` - 解密数据库
- ✅ `POST /api/chat/contacts` - 获取联系人列表
- ✅ `GET /api/chat/contacts/:wxid` - 获取联系人详情
- ✅ `POST /api/chat/msg/count` - 获取消息数量统计
- ✅ `POST /api/chat/msg/list` - 获取消息列表
- ✅ `POST /api/chat/msg/search` - 搜索消息
- ✅ `POST /api/export/csv` - 导出CSV格式
- ✅ `POST /api/export/json` - 导出JSON格式
- ✅ `POST /api/export/html` - 导出HTML格式
- ✅ `POST /api/export/dedb` - 导出解密后的数据库
- ✅ `POST /api/export/endb` - 导出加密的数据库
- ✅ `GET /swagger-ui/*` - Swagger UI API文档
- ✅ `GET /api-doc/openapi.json` - OpenAPI规范文档
- ✅ `GET /health` - 健康检查

#### 前端功能
- ✅ 首页（显示微信信息、获取微信信息按钮、多进程信息展示）
- ✅ 聊天记录页面（消息列表展示、搜索、筛选）
- ✅ 联系人页面（联系人列表、详情查看）
- ✅ 收藏页面（收藏列表展示）
- ✅ 朋友圈页面（朋友圈动态展示）
- ✅ 统计分析页面（数据统计和可视化）
- ✅ 数据导出页面（支持多种格式导出）
- ✅ 工具页面（数据库合并、清理等工具）
- ✅ 设置页面（配置管理）
- ✅ 基础框架（Vue 3 + Vite 8 + UnoCSS + Pinia）
- ✅ API 封装（Axios 配置、API 接口封装、错误处理）
- ✅ **骨架屏组件**（SkeletonLoader，提升加载体验）
- ✅ **图片懒加载指令**（使用Intersection Observer API实现按需加载）
- ✅ **虚拟滚动支持**（@tanstack/vue-virtual依赖）

#### 测试与文档
- ✅ 单元测试覆盖（核心模块、数据库模块、文件版本、内存操作、内存映射、联系人、收藏、媒体、朋友圈、消息查询等）
- ✅ OpenAPI文档（Swagger UI集成）

#### 开发工具
- ✅ 自动提交脚本（PowerShell/Batch/Bash）
- ✅ Git 配置（SSH 方式）

</details>

## 技术栈

### 后端
- **Rust** - 系统级编程语言，高性能
- **Axum** - 异步 Web 框架
- **windows-rs** - Windows API 绑定
- **rusqlite** - SQLite 数据库
- **utoipa** - OpenAPI文档生成
- **utoipa-swagger-ui** - Swagger UI集成
- **prost** - Protobuf解析库

### 前端
- **Vue 3** - 渐进式前端框架
- **Vite 8** - 下一代前端构建工具
- **UnoCSS** - 原子化 CSS 引擎
- **Pinia** - 状态管理
- **Axios** - HTTP客户端
- **@tanstack/vue-virtual** - 虚拟滚动支持

## 项目结构

```
wx-dump-4/
├── backend/                 # Rust 后端
│   ├── src/
│   │   ├── api/            # API 路由
│   │   │   ├── wx_info/    # 微信信息API
│   │   │   ├── decrypt/    # 数据库解密API
│   │   │   ├── chat/       # 聊天记录API
│   │   │   ├── statistics/ # 统计分析API
│   │   │   ├── media/      # 媒体文件API
│   │   │   ├── export/     # 数据导出API
│   │   │   ├── favorite/   # 收藏API
│   │   │   ├── moments/    # 朋友圈API
│   │   │   ├── merge/      # 数据库合并API
│   │   │   ├── cleanup/    # 清理工具API
│   │   │   ├── tools/      # 工具API
│   │   │   └── openapi.rs  # OpenAPI文档
│   │   ├── core/           # 核心功能
│   │   │   ├── process.rs  # 进程管理
│   │   │   ├── memory.rs   # 内存操作
│   │   │   ├── memory_map.rs # 内存映射
│   │   │   ├── version.rs  # 版本适配
│   │   │   ├── version_detection.rs # 版本偏移量自动检测
│   │   │   ├── wx_info.rs  # 微信信息获取
│   │   │   ├── decryption.rs # 数据库解密
│   │   │   └── file_version.rs # 文件版本信息
│   │   ├── db/             # 数据库处理
│   │   │   ├── contact.rs  # 联系人处理
│   │   │   ├── msg.rs      # 消息处理
│   │   │   ├── media.rs    # 媒体处理
│   │   │   ├── favorite.rs # 收藏处理
│   │   │   ├── sns.rs      # 朋友圈处理
│   │   │   ├── bytes_extra.rs # BytesExtra解析
│   │   │   ├── protobuf_parser.rs # Protobuf解析器
│   │   │   └── ...
│   │   ├── models/         # 数据模型
│   │   ├── config/         # 配置管理
│   │   └── utils/          # 工具函数
│   └── Cargo.toml
├── frontend/               # Vue 3 前端
│   ├── src/
│   │   ├── views/          # 页面
│   │   │   ├── Home.vue    # 首页
│   │   │   ├── Chat.vue    # 聊天记录
│   │   │   ├── Contacts.vue # 联系人
│   │   │   ├── Favorite.vue # 收藏
│   │   │   ├── Moments.vue # 朋友圈
│   │   │   ├── Statistics.vue # 统计分析
│   │   │   ├── Export.vue  # 数据导出
│   │   │   ├── Tools.vue  # 工具
│   │   │   ├── Cleanup.vue # 清理
│   │   │   └── Settings.vue # 设置
│   │   ├── components/     # 组件
│   │   │   ├── SkeletonLoader.vue # 骨架屏组件
│   │   │   └── ...
│   │   ├── directives/     # 指令
│   │   │   └── lazyLoad.js # 图片懒加载指令
│   │   ├── api/            # API 调用
│   │   └── router/         # 路由
│   └── package.json
├── auto-commit.ps1         # 自动提交脚本
├── auto-commit.bat
├── auto-commit.sh
├── README.md
└── USAGE.md
```

## 开发

### 后端开发

```bash
cd backend
cargo run
```

后端服务运行在 `http://localhost:3000`

### 前端开发

```bash
cd frontend
npm install
npm run dev
```

前端服务运行在 `http://localhost:5173`

## 计划开发进度

### 待开发功能 🔄

#### 数据导出增强
- [ ] 批量导出（支持导出多个会话）
- [ ] 导出文件压缩支持（ZIP格式）

#### 统计分析增强
- [ ] 词云图前端展示组件（后端API已完成）
- [ ] 更多可视化图表类型（饼图、散点图等，已有折线图和柱状图）
- [ ] 数据导出为图片/PDF

#### 功能增强
- [x] 自动检测版本偏移量（无需手动配置）✅
- [ ] 更多微信 4.0+ 版本支持（完善版本兼容性）
- [ ] 文件版本信息自动识别（自动匹配偏移量）
- [x] 性能优化（内存搜索性能优化、数据库索引优化、缓存机制）✅
- [x] 错误处理增强（更完善的错误提示和处理）✅

#### 界面优化
- [ ] 聊天记录页面完善（聊天界面 UI/UX 优化）
- [x] 设置页面功能（配置管理、主题切换等）✅
- [x] 响应式设计优化（移动端适配，基础支持已完成）✅
- [x] 暗色主题支持 ✅

## 注意事项

1. **权限要求**：需要管理员权限运行（用于读取进程内存）
2. **系统要求**：仅支持 Windows 系统
3. **版本支持**：基础支持 4.0+，支持自动检测版本偏移量，也支持手动配置偏移量
4. **仅供学习交流使用**：本项目仅供学习交流使用，请勿用于非法用途

## 故障排除

<details>
<summary>点击展开查看故障排除指南</summary>

### 问题：无法获取微信信息

**解决方案**：
1. 确保微信已启动并登录
2. 确保以管理员权限运行程序
3. 检查微信版本是否支持（4.0+）

### 问题：数据库解密失败

**解决方案**：
1. 确保已成功获取微信密钥
2. 检查数据库文件是否存在
3. 确认数据库文件未被其他程序占用

### 问题：前端无法连接后端

**解决方案**：
1. 确保后端服务已启动（`http://localhost:3000`）
2. 检查前端配置中的 API 地址
3. 查看浏览器控制台的错误信息

### 问题：编译错误

**解决方案**：
1. 确保 Rust 工具链已正确安装
2. 运行 `cargo clean` 清理缓存
3. 检查依赖版本是否兼容

</details>

## 更新日志

查看完整的提交历史：[Git提交记录](https://github.com/baiyajin/wx-dump-4/commits/main)

<details>
<summary>点击展开查看最近更新</summary>

### 已完成 ✅
- [x] 项目基础结构
- [x] Rust后端核心框架
- [x] Vue3前端基础框架
- [x] 微信信息获取（基础）
- [x] 数据库解密（基础）
- [x] Git配置和自动提交脚本
- [x] 版本偏移量配置导入
- [x] **版本偏移量自动检测**（通过内存搜索特征码自动定位偏移量）
- [x] **Protobuf解析器**（解析微信消息中的BytesExtra字段）
- [x] **数据库查询优化**（索引优化、缓存机制）
- [x] **错误处理增强**（扩展错误类型，用户友好的错误消息）
- [x] **OpenAPI文档集成**（Swagger UI）
- [x] **前端体验优化**（骨架屏组件、图片懒加载、虚拟滚动支持）
- [x] **单元测试完善**（核心模块、数据库模块全面测试覆盖）
- [x] **聊天记录功能**（读取、展示、搜索、筛选）
- [x] **数据导出功能**（CSV、JSON、HTML格式导出）
- [x] **统计分析功能**（数据统计、可视化图表、活跃度分析、联系人分析）
- [x] **界面优化**（设置页面、响应式设计、暗色主题支持）

### 进行中 🔄
- [ ] 批量导出功能（支持导出多个会话）
- [ ] 统计分析功能增强（词云图前端展示、更多图表类型）
- [ ] 数据导出增强（ZIP压缩、图片/PDF导出）

### 待开始 ⏳
- [ ] 更多微信版本支持（完善版本兼容性）
- [ ] 文件版本信息自动识别（自动匹配偏移量）
- [ ] 聊天记录页面UI/UX优化

</details>

## 许可证

本项目采用MIT许可证。

## 贡献

欢迎提交Issue和Pull Request！

## 支持项目

如果这个项目对你有帮助，欢迎通过微信赞赏支持开发者继续改进项目！

<div align="center">
  <img src="wechat_reward.jpg" alt="微信赞赏码" width="300">
  <p><em>你的鼓励是我改BUG的动力 💪</em></p>
</div>

## 技术支持

遇到问题需要帮助？欢迎加入付费技术支持咨询交流QQ群，与开发者和其他用户交流！

<div align="center">
  <img src="qq_group.jpg" alt="付费技术支持咨询交流QQ群" width="300">
  <p><em>扫码加入QQ群，获取技术支持 💬</em></p>
</div>

## 相关链接

- [Git提交历史](https://github.com/baiyajin/wx-dump-4/commits/main)
- [GitHub仓库](https://github.com/baiyajin/wx-dump-4)
