# 自动提交脚本 (PowerShell)
# 使用方法: .\auto-commit.ps1 "提交信息"

param(
    [Parameter(Mandatory=$false)]
    [string]$message = "Auto commit: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
)

Write-Host "开始自动提交..." -ForegroundColor Green

# 检查是否有未提交的更改
$status = git status --porcelain
if ([string]::IsNullOrWhiteSpace($status)) {
    Write-Host "没有需要提交的更改" -ForegroundColor Yellow
    exit 0
}

Write-Host "检测到以下更改:" -ForegroundColor Cyan
git status --short

# 添加所有更改
Write-Host "`n添加所有更改到暂存区..." -ForegroundColor Cyan
git add .

# 提交
Write-Host "提交更改..." -ForegroundColor Cyan
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

