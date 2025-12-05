# 小改动自动提交脚本 (PowerShell)
# 使用方法: .\auto-commit-small.ps1 "提交信息"
# 或者直接运行: .\auto-commit-small.ps1 (会自动生成提交信息)

param(
    [Parameter(Mandatory=$false)]
    [string]$message = ""
)

Write-Host "检查小改动..." -ForegroundColor Green

# 检查是否有未提交的更改
$status = git status --porcelain
if ([string]::IsNullOrWhiteSpace($status)) {
    Write-Host "没有需要提交的更改" -ForegroundColor Yellow
    exit 0
}

# 分析改动类型
$changedFiles = git diff --name-only --cached
$unstagedFiles = git diff --name-only
$allFiles = if ($changedFiles) { $changedFiles } else { $unstagedFiles }

if ([string]::IsNullOrWhiteSpace($allFiles)) {
    Write-Host "没有检测到文件改动" -ForegroundColor Yellow
    exit 0
}

# 统计改动
$addedFiles = @()
$modifiedFiles = @()
$deletedFiles = @()

foreach ($file in $allFiles) {
    $statusLine = git status --porcelain | Where-Object { $_ -match $file }
    if ($statusLine -match '^\?\?') {
        $addedFiles += $file
    } elseif ($statusLine -match '^ D') {
        $deletedFiles += $file
    } else {
        $modifiedFiles += $file
    }
}

# 生成提交信息
if ([string]::IsNullOrWhiteSpace($message)) {
    $fileCount = $allFiles.Count
    $mainFile = $allFiles[0]
    $fileType = if ($mainFile -match '\.rs$') { "Rust" }
                elseif ($mainFile -match '\.(js|ts|vue)$') { "Frontend" }
                elseif ($mainFile -match '\.(md|txt)$') { "Docs" }
                elseif ($mainFile -match '\.(toml|json|yaml|yml)$') { "Config" }
                else { "Code" }
    
    if ($fileCount -eq 1) {
        $fileName = Split-Path -Leaf $mainFile
        $message = "Update ${fileType}: $fileName"
    } else {
        $message = "Update ${fileType}: $fileCount files"
    }
}

Write-Host "`n检测到以下更改:" -ForegroundColor Cyan
git status --short

# 添加所有更改
Write-Host "`n添加所有更改到暂存区..." -ForegroundColor Cyan
git add .

# 提交
Write-Host "提交更改: $message" -ForegroundColor Cyan
git commit -m $message

if ($LASTEXITCODE -eq 0) {
    Write-Host "提交成功!" -ForegroundColor Green
    
    # 推送到远程
    Write-Host "推送到远程仓库..." -ForegroundColor Cyan
    git push origin main
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "推送成功!" -ForegroundColor Green
    } else {
        Write-Host "推送失败，请检查网络连接和SSH配置" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "提交失败" -ForegroundColor Red
    exit 1
}

Write-Host "`n完成!" -ForegroundColor Green

