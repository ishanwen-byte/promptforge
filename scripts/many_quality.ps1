# 复杂度检查
Write-Host "[INFO] Running code complexity analysis..."
try {
    # 检查 howmany 是否安装
    if (-not (Get-Command howmany -ErrorAction SilentlyContinue)) {
        Write-Host "[WARNING] howmany not installed. Install with: cargo install howmany"
        Write-Host "[WARNING] Skipping complexity analysis. Consider installing it for better code quality checks."
    } else {
        # 设置复杂度阈值
        $cyclomaticErrorThreshold = 15
        $cognitiveErrorThreshold = 10
        $functionLengthErrorThreshold = 100
        
        # 临时文件存储JSON输出
        $tempFile = [System.IO.Path]::GetTempFileName()
        
        try {
            # 运行 howmany 并输出JSON
            howmany . --format json --output $tempFile
            
            # 读取并解析JSON
            $metrics = Get-Content $tempFile | ConvertFrom-Json
            
            # 检查高复杂度函数
            $highCyclomatic = @()
            $highCognitive = @()
            $highLength = @()
            
            # 遍历所有函数进行检查
            foreach ($file in $metrics.files) {
                foreach ($function in $file.functions) {
                    if ($function.cyclomatic -gt $cyclomaticErrorThreshold) {
                        $highCyclomatic += @{
                            Name = $function.name
                            File = $file.path
                            Complexity = $function.cyclomatic
                        }
                    }
                    
                    if ($function.cognitive -gt $cognitiveErrorThreshold) {
                        $highCognitive += @{
                            Name = $function.name
                            File = $file.path
                            Complexity = $function.cognitive
                        }
                    }
                    
                    if ($function.lines -gt $functionLengthErrorThreshold) {
                        $highLength += @{
                            Name = $function.name
                            File = $file.path
                            Lines = $function.lines
                        }
                    }
                }
            }
            
            # 报告结果
            $hasErrors = $false
            
            if ($highCyclomatic.Count -gt 0) {
                Write-Host "[ERROR] Functions with high cyclomatic complexity (>$cyclomaticErrorThreshold):"
                foreach ($func in $highCyclomatic) {
                    Write-Host "  - $($func.Name) in $($func.File) ($($func.Complexity))"
                }
                $hasErrors = $true
            }
            
            if ($highCognitive.Count -gt 0) {
                Write-Host "[ERROR] Functions with high cognitive complexity (>$cognitiveErrorThreshold):"
                foreach ($func in $highCognitive) {
                    Write-Host "  - $($func.Name) in $($func.File) ($($func.Complexity))"
                }
                $hasErrors = $true
            }
            
            if ($highLength.Count -gt 0) {
                Write-Host "[ERROR] Functions with excessive length (>$functionLengthErrorThreshold lines):"
                foreach ($func in $highLength) {
                    Write-Host "  - $($func.Name) in $($func.File) ($($func.Lines) lines)"
                }
                $hasErrors = $true
            }
            
            if (-not $hasErrors) {
                Write-Host "[OK] Complexity analysis passed. All functions within acceptable limits."
            } else {
                exit 1
            }
        } finally {
            # 清理临时文件
            if (Test-Path $tempFile) {
                Remove-Item $tempFile -Force
            }
        }
    }
} catch {
    Write-Host "[ERROR] Complexity analysis failed: $_"
    exit 1
}