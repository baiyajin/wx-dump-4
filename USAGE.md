# 使用说明

## Git SSH 配置

项目已配置为使用 SSH 方式连接 GitHub。

### 检查 SSH 配置

```bash
git remote -v
```

应该显示：
```
origin  git@github.com:baiyajin/wx-dump-4.git (fetch)
origin  git@github.com:baiyajin/wx-dump-4.git (push)
```

### 如果还没有配置 SSH 密钥

1. 生成 SSH 密钥：
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

2. 将公钥添加到 GitHub：
```bash
cat ~/.ssh/id_ed25519.pub
```
复制输出内容，添加到 GitHub Settings > SSH and GPG keys

3. 测试连接：
```bash
ssh -T git@github.com
```

## 自动提交脚本

项目提供了三个平台的自动提交脚本：

### Windows PowerShell
```powershell
.\auto-commit.ps1 "提交信息"
```

### Windows CMD
```cmd
auto-commit.bat "提交信息"
```

### Linux/Mac
```bash
chmod +x auto-commit.sh
./auto-commit.sh "提交信息"
```

### 使用默认提交信息
如果不提供提交信息，脚本会自动生成带时间戳的提交信息：
```
Auto commit: 2024-01-01 12:00:00
```

## 开发流程

1. 修改代码
2. 运行自动提交脚本
3. 脚本会自动：
   - 检测更改
   - 添加到暂存区
   - 提交更改
   - 推送到远程仓库

## 注意事项

- 确保已配置 SSH 密钥
- 确保有推送权限
- 脚本会在推送失败时提示检查网络和SSH配置

