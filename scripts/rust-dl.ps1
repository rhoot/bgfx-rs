$Channel = "http://static.rust-lang.org/dist/channel-rust-$env:CHANNEL"
Write-Output "reading: $Channel"

$Manifest      = Invoke-WebRequest -Uri $Channel
$ManifestLines = $Manifest.RawContent -Split "`n"

foreach ($Line in $ManifestLines -Like "*$env:TARGET.exe") {
    $Url  = "http://static.rust-lang.org/dist/$Line"
    $Path = "$PSScriptRoot\rust.exe"
    Write-Output "downloading:"
    Write-Output "  source: $Url"
    Write-Output "  target: $Path"
    (New-Object System.Net.WebClient).DownloadFile($Url, $Path)
}
