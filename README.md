# Missions
## 概要
ゲームによくある```デイリーミッション```の感覚で日々のタスクや勉強の習慣化を促すアプリケーション  
**最大7個**のミッションを設定することができ、完了すると(```Complete```)2expを取得することができる  
日本時間0:00に```Complete```がリセットされる

![img](./docs/img/home.png)
## Requirements
- Docker engine
- Docker compose
## Run
### 1. ```mysql_config```ファイルの設定
```mysql_config```ファイルの作成
```
touch mysql_config
```
```mysql_config```ファイル内でパスワードなどの設定
```
MYSQL_ROOT_PASSWORD=example_password
MYSQL_USER=example
MYSQL_PASSWORD=example_pass
MYSQL_DATABASE=missions_db
MYSQL_ALLOW_EMPTY_PASSWORD=false
```
### 2. ```.env```ファイルの設定
```
touch .env
```
```diff
# CORSポリシーのために設定
# Docker composeで起動する際は適当な値でも動作する(reverse_proxyでsame domainのため)
ALLOW_ORIGIN=http://localhost:3000

# mysql_configで設定した値を参照
DATABASE_URL=mysql://example:example_pass@localhost:13306/missions_db

# exp_table.csvまでのファイルパス
FILE_PATH=/home/my_user/develop/missions-systems/exp_table.csv
```
### 3. Docker
コンテナの起動
```
docker compose up
```
## 使い方
http://localhost/
にアクセスする
### SignUp
ユーザー登録
- Emailはユニークかつ正しくないと登録できない
- パスワードは８文字以上15文字以下
- 初回は自動ログインされる
![img](./docs/img/signup.png)
### Login
![img](./docs/img/login.png)
### Missionの追加
```Add```タブを選択し追加
![img](./docs/img/add.png)
### Home画面
- ミッションの確認
- ミッション名の変更
- ミッションの詳細の変更
- 完了にセット
![img](./docs/img/home.png)
### 経験値/レベルの確認
- Statusタブを選択
- 現在のレベルの確認
- 現在の経験値の確認
- レベルアップに必要な経験値量
![img](./docs/img/level.png)
### ユーザー名の変更/削除
- ヘッダーのアイコンボタンをクリック
![img](./docs/img/user.png)
