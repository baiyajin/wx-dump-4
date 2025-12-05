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

#### API 接口
- ✅ `POST /api/wx/info` - 获取微信信息
- ✅ `POST /api/wx/decrypt` - 解密数据库
- ✅ `GET /health` - 健康检查

#### 前端功能
- ✅ 首页（显示微信信息、获取微信信息按钮、多进程信息展示）
- ✅ 基础框架（Vue 3 + Vite 8 + UnoCSS + Pinia）
- ✅ API 封装（Axios 配置、API 接口封装、错误处理）

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

### 前端
- **Vue 3** - 渐进式前端框架
- **Vite 8** - 下一代前端构建工具
- **UnoCSS** - 原子化 CSS 引擎
- **Pinia** - 状态管理

## 项目结构

```
wx-dump-4/
├── backend/                 # Rust 后端
│   ├── src/
│   │   ├── api/            # API 路由
│   │   ├── core/           # 核心功能
│   │   │   ├── process.rs  # 进程管理
│   │   │   ├── memory.rs   # 内存操作
│   │   │   ├── version.rs  # 版本适配
│   │   │   ├── wx_info.rs  # 微信信息获取
│   │   │   ├── decryption.rs # 数据库解密
│   │   │   └── file_version.rs # 文件版本信息
│   │   ├── db/             # 数据库处理
│   │   ├── models/         # 数据模型
│   │   ├── config/         # 配置管理
│   │   └── utils/          # 工具函数
│   └── Cargo.toml
├── frontend/               # Vue 3 前端
│   ├── src/
│   │   ├── views/          # 页面
│   │   ├── components/     # 组件
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

#### 聊天记录相关
- [ ] 聊天记录读取（从解密后的数据库读取聊天记录）
- [ ] 聊天记录展示（在 Web 界面中展示聊天记录）
- [ ] 聊天记录搜索（支持关键词搜索）
- [ ] 聊天记录筛选（按时间、联系人、消息类型筛选）

#### 数据导出
- [ ] HTML 格式导出（支持富文本格式）
- [ ] CSV 格式导出（支持 Excel 打开）
- [ ] JSON 格式导出（支持程序化处理）
- [ ] 批量导出（支持导出多个会话）

#### 统计分析
- [ ] 聊天数据统计（消息数量、发送时间分布等）
- [ ] 可视化图表（折线图、柱状图、词云等）
- [ ] 聊天活跃度分析（每日/每周/每月活跃度）
- [ ] 联系人分析（聊天频率、消息类型分布）

#### 功能增强
- [ ] 自动检测版本偏移量（无需手动配置）
- [ ] 更多微信 4.0+ 版本支持（完善版本兼容性）
- [ ] 文件版本信息自动识别（自动匹配偏移量）
- [ ] 性能优化（内存搜索性能优化）
- [ ] 错误处理增强（更完善的错误提示和处理）

#### 界面优化
- [ ] 聊天记录页面完善（聊天界面 UI/UX）
- [ ] 设置页面功能（配置管理、主题切换等）
- [ ] 响应式设计优化（移动端适配）
- [ ] 暗色主题支持

## 注意事项

1. **权限要求**：需要管理员权限运行（用于读取进程内存）
2. **系统要求**：仅支持 Windows 系统
3. **版本支持**：基础支持 4.0+，需要添加具体版本的偏移量
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

- ✅ **docs: 添加 logo、支持部分和 QQ 群二维码到 README** - 完善 README 文档，添加项目 logo 和支持信息
- ✅ **feat: 添加文件版本读取、导入 WX_OFFS 配置、修复编译错误** - 实现文件版本信息读取功能，导入微信偏移量配置
- ✅ **docs: 添加项目总结和文档** - 添加项目总结文档，记录已完成的工作
- ✅ **fix: 修复编译错误并添加使用文档** - 修复编译问题，添加使用说明文档
- ✅ **chore: 初始化项目结构** - Rust 后端 + Vue3 前端项目结构初始化

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
