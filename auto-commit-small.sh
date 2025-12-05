#!/bin/bash
# 小改动自动提交脚本 (Bash)
# 使用方法: ./auto-commit-small.sh "提交信息"
# 或者直接运行: ./auto-commit-small.sh (会自动生成提交信息)

MESSAGE="${1:-}"

echo "检查小改动..."

# 检查是否有未提交的更改
if [ -z "$(git status --porcelain)" ]; then
    echo "没有需要提交的更改"
    exit 0
fi

# 分析改动类型
CHANGED_FILES=$(git diff --name-only --cached 2>/dev/null)
UNSTAGED_FILES=$(git diff --name-only 2>/dev/null)
ALL_FILES="${CHANGED_FILES}${UNSTAGED_FILES}"

if [ -z "$ALL_FILES" ]; then
    echo "没有检测到文件改动"
    exit 0
fi

# 统计改动
FILE_COUNT=$(echo "$ALL_FILES" | wc -l | tr -d ' ')
MAIN_FILE=$(echo "$ALL_FILES" | head -n 1)

# 生成提交信息
if [ -z "$MESSAGE" ]; then
    if [[ "$MAIN_FILE" == *.rs ]]; then
        FILE_TYPE="Rust"
    elif [[ "$MAIN_FILE" == *.js ]] || [[ "$MAIN_FILE" == *.ts ]] || [[ "$MAIN_FILE" == *.vue ]]; then
        FILE_TYPE="Frontend"
    elif [[ "$MAIN_FILE" == *.md ]] || [[ "$MAIN_FILE" == *.txt ]]; then
        FILE_TYPE="Docs"
    elif [[ "$MAIN_FILE" == *.toml ]] || [[ "$MAIN_FILE" == *.json ]] || [[ "$MAIN_FILE" == *.yaml ]] || [[ "$MAIN_FILE" == *.yml ]]; then
        FILE_TYPE="Config"
    else
        FILE_TYPE="Code"
    fi
    
    if [ "$FILE_COUNT" -eq 1 ]; then
        FILE_NAME=$(basename "$MAIN_FILE")
        MESSAGE="Update $FILE_TYPE: $FILE_NAME"
    else
        MESSAGE="Update $FILE_TYPE: $FILE_COUNT files"
    fi
fi

echo ""
echo "检测到以下更改:"
git status --short

# 添加所有更改
echo ""
echo "添加所有更改到暂存区..."
git add .

# 提交
echo "提交更改: $MESSAGE"
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

