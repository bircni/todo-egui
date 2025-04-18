# Enable push with follow tags
git config --global push.followTags true

# Check if git cliff is installed
if (-not (Get-Command git-cliff -ErrorAction SilentlyContinue)) {
    Write-Host "git cliff is not installed. Installing it now..."
    # Install git cliff using Winget
    winget install -e --id orhun.git-cliff -s winget
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to install git cliff. Please install it manually."
        exit 1
    }
    Write-Host "git cliff installed successfully."
}
# Check if cargo-verset is installed
if (-not (Get-Command cargo-verset -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-verset is not installed. Please install it with cargo install cargo-verset"
    exit 1
}

$version = git cliff --bumped-version

Write-Host "Calculated version: $version"
Write-Host "Updating version in Cargo.toml..."
# Update the version in Cargo.toml
cargo verset -v $version
Write-Host "Version updated successfully in Cargo.toml."
# Generate the changelog
Write-Host "Generating changelog..."
git cliff --output CHANGELOG.md -t $version
Write-Host "Changelog generated successfully."
# Commit the changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore($version): release $version"
git tag -a $version -m "Release $version" 