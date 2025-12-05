# 项目总结

## 已完成的工作

### 1. Rust 后端实现 ✅

#### 核心模块
- **进程管理** (`core/process.rs`)
  - 进程枚举
  - 进程句柄管理
  - 查找 WeChat.exe 进程

- **内存操作** (`core/memory.rs`)
  - 内存读取
  - 内存搜索
  - 内存映射查询
  - 字符串和指针读取

- **版本适配** (`core/version.rs`)
  - 版本号解析
  - 地址长度检测（支持 4.0+）
  - 偏移量管理

- **微信信息获取** (`core/wx_info.rs`)
  - 获取微信账号、昵称、手机号等信息
  - 获取微信密钥
  - 获取微信目录路径
  - 支持多进程（微信多开）

- **数据库解密** (`core/decryption.rs`)
  - SQLite 数据库解密
  - 支持批量解密
  - AES-256-CBC 解密算法

#### API 接口
- `POST /api/wx/info` - 获取微信信息
- `POST /api/wx/decrypt` - 解密数据库
- `GET /health` - 健康检查

### 2. Vue 3 前端实现 ✅

#### 项目结构
- Vite 8 构建配置
- UnoCSS 原子化 CSS
- Vue Router 路由
- Pinia 状态管理（已配置）

#### 页面
- **首页** (`views/Home.vue`)
  - 显示微信信息
  - 获取微信信息按钮
  - 多进程信息展示

- **聊天记录** (`views/Chat.vue`)
  - 占位页面（待开发）

- **设置** (`views/Settings.vue`)
  - 占位页面（待开发）

#### API 封装
- Axios 配置
- API 接口封装
- 错误处理

### 3. Git 配置 ✅

- ✅ 已配置为 SSH 方式
- ✅ 远程仓库：`git@github.com:baiyajin/wx-dump-4.git`

### 4. 自动提交脚本 ✅

提供了三个平台的脚本：
- `auto-commit.ps1` - PowerShell (Windows)
- `auto-commit.bat` - Batch (Windows CMD)
- `auto-commit.sh` - Bash (Linux/Mac)

功能：
- 自动检测更改
- 添加到暂存区
- 提交更改
- 推送到远程仓库

## 项目结构

```
wx-dump-4/
├── backend/                 # Rust 后端
│   ├── src/
│   │   ├── api/            # API 路由
│   │   ├── core/           # 核心功能
│   │   ├── db/             # 数据库处理
│   │   ├── models/         # 数据模型
│   │   ├── config/         # 配置管理
│   │   └── utils/          # 工具函数
│   └── Cargo.toml
├── frontend/               # Vue 3 前端
│   ├── src/
│   │   ├── views/          # 页面
│   │   ├── components/    # 组件
│   │   ├── api/            # API 调用
│   │   └── router/         # 路由
│   └── package.json
├── auto-commit.ps1         # 自动提交脚本
├── auto-commit.bat
├── auto-commit.sh
├── README.md
└── USAGE.md
```

## 技术栈

### 后端
- Rust 2021 Edition
- Axum 0.7 - Web 框架
- windows-rs 0.52 - Windows API
- rusqlite 0.30 - SQLite 数据库
- AES/CBC - 加密解密

### 前端
- Vue 3.4
- Vite 8.0
- UnoCSS 0.58
- Pinia 2.1
- Axios 1.6

## 支持的功能

### 已实现 ✅
- [x] 获取微信进程信息
- [x] 读取内存获取微信信息
- [x] 支持微信 4.0+ 版本（地址长度检测）
- [x] 数据库解密
- [x] Web API 接口
- [x] 前端界面基础框架

### 待开发 🔄
- [ ] 聊天记录读取
- [ ] 聊天记录展示
- [ ] 数据导出（HTML/CSV/JSON）
- [ ] 统计分析
- [ ] 文件版本信息读取
- [ ] 自动检测版本偏移量
- [ ] 更多微信 4.0+ 版本支持

## 使用说明

### 启动后端
```bash
cd backend
cargo run
```

### 启动前端
```bash
cd frontend
npm install
npm run dev
```

### 自动提交
```powershell
.\auto-commit.ps1 "提交信息"
```

## 注意事项

1. **权限要求**：需要管理员权限运行（读取进程内存）
2. **系统要求**：仅支持 Windows 系统
3. **版本支持**：基础支持 4.0+，需要添加具体版本的偏移量
4. **SSH 配置**：确保已配置 GitHub SSH 密钥

## 下一步计划

1. 完善版本偏移量配置（从原项目导入）
2. 实现文件版本信息读取
3. 实现聊天记录读取和展示
4. 添加数据导出功能
5. 优化内存搜索性能
6. 添加更多错误处理

## 提交记录

- Initial commit: Rust backend + Vue3 frontend project structure
- Fix compilation errors and add usage documentation
- Test auto-commit script

