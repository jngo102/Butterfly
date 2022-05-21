[English](./README.md) | 简体中文

<h1 align='center'><b>Butterfly</b></h1>
<p align='center'>
    <a href="https://github.com/jngo102/Butterfly/actions/workflows/main.yml">
        <img src="https://img.shields.io/github/workflow/status/jngo102/Butterfly/Publish" 
             alt="">
    </a>
    <a href="https://github.com/jngo102/Butterfly">
        <img src="https://img.shields.io/github/downloads/jngo102/Butterfly/total" 
             alt="">
    </a>
    <a href="https://github.com/jngo102/Butterfly/commits">
        <img src="https://img.shields.io/github/commit-activity/m/jngo102/Butterfly"
             alt="">
    </a>
    <a href="https://github.com/jngo102/Butterfly/blob/main/LICENSE.md">
        <img src="https://img.shields.io/github/license/jngo102/Butterfly"
             alt="">
    </a>
</p>
<p align='center'>
    <a href="https://discord.gg/VDsg3HmWuB">
        <img src="https://img.shields.io/discord/879125729936298015?logo=discord"
            alt="">
    </a>
    <a href="https://twitter.com/intent/follow?screen_name=JngoCreates">
        <img src="https://img.shields.io/twitter/follow/JngoCreates?style=social&logo=twitter"
             alt="">
    </a>
</p>

<p align='center'>
    <img src='./src-tauri/icons/icon.png' 
         alt='' 
         width='192'
         height='192'/>
</p>



这是一个使用[**Tauri**](https://github.com/tauri-apps/tauri/)和[**VueJS**](https://github.com/vuejs/core/)制作的Hollow Knight Mod安装器

![Butterfly screenshot](/images/window.png)

## **安装**
### Windows
1. 转跳到[下载](https://github.com/jngo102/Butterfly/releases)页面
2. 下载`butterfly_{version}_x64_en-US.msi`文件
3. 下载完成后双击该文件
4. 根据安装向导安装Butterfly，大多数选项可以保持默认值

### macOS
1. 转跳到[下载](https://github.com/jngo102/Butterfly/releases)页面.
2. 下载`butterfly_{version}_x64.dmg`文件
3. 下载完成后双击该文件
4. 根据安装向导安装Butterfly

### Linux
1. 转跳到[下载](https://github.com/jngo102/Butterfly/releases)页面.
2. 您将下载的文件取决于您使用的包管理器。 有关如何为您的系统安装软件包的信息，请参阅您的发行版手册。 对于 Ubuntu 和 Debian 发行版，这是 `butterfly_{version}_amd64.deb`。
3. 在您的文件系统中打开下载位置。
4. 使用你的包管理器，安装你下载的包。

## **从源代码编译**
1. 确保系统上安装了 NodeJS。 可以在他们的 [网站](https://nodejs.org/en/download/) 上找到它的安装程序。
2. 如果尚未安装，请使用命令 `npm install --global yarn` 安装 Yarn 包管理器。
3. 克隆此存储库。
4. 运行命令`yarn build`。
5. yarn 构建完成后，运行`yarn tauri build`。
6.输出的可执行文件可以在`{repoPath}/src-tauri/target/{configuration}`中找到，它的安装程序可以在`{repoPath}/src-tauri/target/{configuration}/bundle/{installerType}`中找到

## **故障排除**
* 在[GitHub](https://github.com/jngo102/Butterfly/issues)上发起issue之前，请确保没有人遇到过已解决的类似问题。*

 ### 应用程序打开，但内容显示错误页面。
 >确保您已连接到互联网。 如果没有连接，Butterfly 将无法工作。

 ### 应用程序无法打开，或者它打开并立即关闭。
 >删除设置文件并重新运行应用程序。 对于 Windows，这是在 `%APPDATA%/Butterfly/Settings.json |  C:/Users/{userName}/AppData/Roaming/Butterfly/Settings.json`。 对于 macOS，位于 `$HOME/Library/Application Support/Butterfly/Settings.json |  /Users/{userName}/Library/Application Support/Butterfly/Settings.json`。 对于 Linux，这是 `$HOME/.local/share/Butterfly/Settings.json |  /home/{userName}/.local/share/Butterfly/Settings.json`

 ### 我无法安装或卸载模组。
 >确保游戏没有运行。

 ### 我无法启用/禁用 Modding API
 >[验证完整性](https://help.steampowered.com/en/faqs/view/0C48-FCBD-DA71-93EB) 通过 Steam 下载您的空洞骑士文件，然后重新启动Butterfly

 如果还有其他问题，请尝试刷新应用程序网页（快捷键 Ctrl + R）。

 对于其他问题，如果应用程序可以打开，您可以通过单击应用程序标题中的“报告问题/建议功能”链接来报告问题。 否则，进入[issues page](https://github.com/jngo102/Butterfly/issues?q=is%3Aissue)，点击绿色的`New issue`按钮，选择`🦋 Bug Report`模板使用 绿色的“Get started”按钮。 尽你所能完成这些领域。

 此外，您还可以进入[空洞骑士 Modding Discord 服务器](https://discord.gg/VDsg3HmWuB) 并在 `#support` 频道寻求帮助。

 ## **贡献**
 pr总是受欢迎的。 
请务必遵循 [模板](https://github.com/jngo102/Butterfly/blob/main/.github/PULL_REQUEST_TEMPLATE/pull_request.md)。

 还需要对应用程序的本地化做出贡献。 要添加或编辑语言环境，请导航到 `src/i18n.ts`，单击页面右上角的铅笔图标（工具提示是“Edit file”），然后在 `messages` 变量中翻译字段 JSON 对象。 使用英语字段作为标准。

 如果你想 *添加* 一个新的语言环境，你还需要编辑 `src/App.vue`。 查找 `languages` 和 `languagesMap` 变量并将新语言的名称添加到 `languages` 中，并根据其他语言环境的设置在 `languagesMap` 变量中添加新语言名称和新语言环境名称的键值对，不要忘记冒号和逗号。

 完成后，建议您在页面底部输入一条消息，描述您所做的更改，标题为“Commit changes”。 完成后单击绿色的“Propose changes”按钮，然后在接下来的页面中，单击绿色的“Create pull request”按钮提交更改。

## **许可证**
Butterfly 根据 [GNU 通用公共许可证 v3] (https://www.gnu.org/licenses/gpl-3.0.en.html) 获得许可。 这意味着如果您分叉并修改此代码，如果您希望分发它，您也必须根据 GPLv3 许可您的修改。
