## 手順  
### vscodeのRemote Containerで起動する手順
1. docker-compose.ymlのあるディレクトリ(~/test_rust_2021)をvscodeで開く
2. コマンドパレットを表示: 「Ctrl + Shift + P」
3. Remote Containers:Reopen in Containerを選択  
[(参考リンク)](https://penpen-dev.com/blog/vscode-remote-container-toha/)

### vscodeのRemote Containerで終了する手順
1. コマンドパレットを表示: 「Ctrl + Shift + P」
2. Remote Containers:Reopen Folder Locallyを選択

### rustを実行する方法
1. cargo new --bin helloworld
2. cd testHello1/
3. cargo run

### dockerコマンドで起動する方法
1. docker-compose up -d --build  
2. docker-compose exec rust bash  
not need(docker-compose run rust bash)

## other 
### …or create a new repository on the command line
echo "# test_rust_2021" >> README.md  
git init  
git add README.md  
git commit -m "first commit"  
git branch -M main  
git remote add origin https://github.com/S-okubomy/test_rust_2021.git  
git push -u origin main  

### …or push an existing repository from the command line
git remote add origin https://github.com/S-okubomy/test_rust_2021.git  
git branch -M main  
git push -u origin main  

### …or import code from another repository
You can initialize this repository with code from a Subversion, Mercurial, or TFS project.  