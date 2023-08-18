# poker

#### 介绍
斗地主残局，深度优先搜索
[在线体验](https://zzoe.github.io/)

#### 使用说明
1. 选择对方手牌
2. 选择我方手牌
3. 选择先手方
4. 开始，程序按照我方手牌，计算是否存在必胜解法（不一定是最优解）
5. 存在时，自动打出我方招数
6. 选择对手的应招
7. 循环5-6两步，直至游戏结束

#### 软件架构
软件架构说明
1. poker 残局计算
2. poker_cli 命令行界面
3. poker-web web界面
3.1. poker-client web前端，dioxus
3.2. poker-server web服务端，poem
其它slint和egui没有实现，不想弄了

#### 开发教程
1. 安装nvm-windows:
https://github.com/coreybutler/nvm-windows/releases

2. 安装nodejs
```
nvm install latest
nvm use 版本号
```

3. 安装pnpm：
```
iwr https://get.pnpm.io/install.ps1 -useb | iex
```

4. 修改环境变量
PNPM_HOME
PATH

5. 修改镜像
```
pnpm config set registry https://registry.npm.taobao.org/ 
```

6. 安装 tailwindcss
```
pnpm i -D tailwindcss
```

7. 初始化
```
pnpm tailwindcss init
```

#### web编译教程
1. tailwindcss 在poker-client目录
```
pnpm dev
```

2. dioxus 在poker-client目录
```
dx build --release
```

3. poem 在poker-server目录
```
cargo build --release
```

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
