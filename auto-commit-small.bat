@echo off
REM 小改动自动提交脚本 (Batch)
REM 使用方法: auto-commit-small.bat "提交信息"
REM 或者直接运行: auto-commit-small.bat (会自动生成提交信息)

setlocal enabledelayedexpansion

set MESSAGE=%1

echo 检查小改动...

REM 检查是否有未提交的更改
git status --porcelain >nul 2>&1
if errorlevel 1 (
    echo 没有需要提交的更改
    exit /b 0
)

REM 分析改动类型
git diff --name-only --cached >nul 2>&1
set CHANGED_FILES=%errorlevel%

git diff --name-only >nul 2>&1
set UNSTAGED_FILES=%errorlevel%

REM 生成提交信息
if "%MESSAGE%"=="" (
    for /f "tokens=*" %%f in ('git diff --name-only') do (
        set MAIN_FILE=%%f
        goto :found_file
    )
    :found_file
    
    if "!MAIN_FILE:*.rs=!" neq "!MAIN_FILE!" (
        set FILE_TYPE=Rust
    ) else if "!MAIN_FILE:*.js=!" neq "!MAIN_FILE!" (
        set FILE_TYPE=Frontend
    ) else if "!MAIN_FILE:*.vue=!" neq "!MAIN_FILE!" (
        set FILE_TYPE=Frontend
    ) else if "!MAIN_FILE:*.md=!" neq "!MAIN_FILE!" (
        set FILE_TYPE=Docs
    ) else (
        set FILE_TYPE=Code
    )
    
    for %%f in ("!MAIN_FILE!") do set FILE_NAME=%%~nxf
    set MESSAGE=Update !FILE_TYPE!: !FILE_NAME!
)

echo.
echo 检测到以下更改:
git status --short

REM 添加所有更改
echo.
echo 添加所有更改到暂存区...
git add .

REM 提交
echo 提交更改: %MESSAGE%
git commit -m "%MESSAGE%"

if %errorlevel% equ 0 (
    echo 提交成功!
    
    REM 推送到远程
    echo 推送到远程仓库...
    git push origin main
    
    if %errorlevel% equ 0 (
        echo 推送成功!
    ) else (
        echo 推送失败，请检查网络连接和SSH配置
        exit /b 1
    )
) else (
    echo 提交失败
    exit /b 1
)

echo.
echo 完成!

