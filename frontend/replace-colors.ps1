$files = Get-ChildItem -Path "c:\Users\victo\Desktop\Inmobiliaria\frontend\src\pages", "c:\Users\victo\Desktop\Inmobiliaria\frontend\src\components" -Filter "*.tsx" -Recurse
foreach ($f in $files) {
    $content = Get-Content -Raw -Path $f.FullName
    $newContent = $content -replace '\bbg-white\b', 'bg-card' `
        -replace '\bbg-slate-50\b', 'bg-background' `
        -replace '\bbg-slate-100\b', 'bg-muted' `
        -replace '\btext-slate-900\b', 'text-foreground' `
        -replace '\btext-slate-800\b', 'text-foreground' `
        -replace '\btext-slate-700\b', 'text-foreground' `
        -replace '\btext-slate-600\b', 'text-muted-foreground' `
        -replace '\btext-slate-500\b', 'text-muted-foreground' `
        -replace '\btext-slate-400\b', 'text-muted-foreground' `
        -replace '\btext-slate-300\b', 'text-muted-foreground' `
        -replace '\bborder-slate-100\b', 'border-border' `
        -replace '\bborder-slate-200\b', 'border-border' `
        -replace '\bring-offset-white\b', 'ring-offset-background' `
        -replace '\bring-slate-950\b', 'ring-ring'
    if ($newContent -cne $content) {
        Set-Content -Path $f.FullName -Value $newContent -NoNewline
    }
}
Write-Output "Done"
