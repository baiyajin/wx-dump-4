#!/bin/bash
# 自动提交脚本 (Bash)
# 使用方法: ./auto-commit.sh "提交信息"

MESSAGE="${1:-Auto commit: $(date '+%Y-%m-%d %H:%M:%S')}"

echo "开始自动提交..."

# 检查是否有未提交的更改
if [ -z "$(git status --porcelain)" ]; then
    echo "没有需要提交的更改"
    exit 0
fi

echo "检测到以下更改:"
git status --short

# 添加所有更改
echo ""
echo "添加所有更改到暂存区..."
git add .

# 提交
echo "提交更改..."
git commit -m "$MESSAGE"

if [ $? -eq 0 ]; then
    echo "提交成功!"
    
    # 推送到远程
    echo "推送到远程仓库..."
    git push origin main
    
    if [ $? -eq 0 ]; then
        echo "推送成功!"
    else
        echo "推送失败，请检查网络连接和SSH配置"
        exit 1
    fi
else
    echo "提交失败"
    exit 1
fi

echo ""
echo "完成!"

