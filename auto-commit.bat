@echo off
REM 自动提交脚本 (Windows Batch)
REM 使用方法: auto-commit.bat "提交信息"

setlocal

if "%1"=="" (
    for /f "tokens=2 delims==" %%a in ('wmic os get localdatetime /value') do set datetime=%%a
    set MESSAGE=Auto commit: %datetime:~0,4%-%datetime:~4,2%-%datetime:~6,2% %datetime:~8,2%:%datetime:~10,2%:%datetime:~12,2%
) else (
    set MESSAGE=%1
)

echo 开始自动提交...

REM 检查是否有未提交的更改
git status --porcelain >nul 2>&1
if %errorlevel% neq 0 (
    echo 没有需要提交的更改
    exit /b 0
)

echo 检测到以下更改:
git status --short

REM 添加所有更改
echo.
echo 添加所有更改到暂存区...
git add .

REM 提交
echo 提交更改...
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

