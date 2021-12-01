# Download last release
curl -o opengame -LJO $(curl -s https://api.github.com/repos/Dragnansia/OpenGame/releases | grep browser_download_url | grep '[.]deb' | head -n 1 | cut -d '\"' -f 4)

# Add Execution
chmod +x opengame

# Move file to bin folder
mv opengame ~/bin
