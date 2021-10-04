# 7zip is needed for installer creation (installed at C:\Program Files\7-Zip\7z.exe)
$dir = Split-Path -Path $PSScriptRoot -Leaf
$dest = ".\$dir\target\eliasfl-chess-gui-installer"
Set-Location ..
if (Test-Path "${dest}.exe") {
    Remove-Item "${dest}.exe"
}
& "C:\Program Files\7-Zip\7z.exe" a $dest -t7z -mmt "-sfxC:\Program Files\7-Zip\7z.sfx" $dir\target\debug\eliasfl-chess-gui.exe $dir\assets\
& "C:\Program Files\7-Zip\7z.exe" rn "${dest}.exe" $dir\target\debug\eliasfl-chess-gui.exe $dir\eliasfl-chess-gui.exe
