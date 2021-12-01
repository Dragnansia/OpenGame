echo -e "\x1b[0;32m[Info] Download last opengame release\x1b[0m"
curl -o opengame -LJO "$(curl -s https://api.github.com/repos/Dragnansia/OpenGame/releases | grep browser_download_url | grep 'opengame' | head -n 1 | cut -d '"' -f 4)"

echo -e "\x1b[0;32m[Info] Add execution mode to opengame\x1b[0m"
chmod +x opengame

echo -e "\x1b[0;32m[Info] Move opengame to ~/bin\x1b[0m"
mv opengame ~/bin
