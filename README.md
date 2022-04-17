<h1 align='center'><b>Butterfly</b></h1>
<p align='center'>
    <a href="https://github.com/jngo102/Butterfly/actions/workflows/main.yml">
        <img src="https://img.shields.io/github/workflow/status/jngo102/Butterfly/Publish" 
             alt="Butterfly build status">
    </a>
    <a href="https://github.com/jngo102/Butterfly">
        <img src="https://img.shields.io/github/downloads/jngo102/Butterfly/total" 
             alt="Butterfly build status">
    </a>
    <a href="https://github.com/jngo102/Butterfly/commits">
        <img src="https://img.shields.io/github/commit-activity/m/jngo102/Butterfly"
             alt="Butterfly commit frequency">
    </a>
    <a href="https://github.com/jngo102/Butterfly/blob/main/LICENSE.md">
        <img src="https://img.shields.io/github/license/jngo102/Butterfly"
             alt="Butterfly software license">
    </a>
</p>
<p align='center'>
    <a href="https://discord.gg/VDsg3HmWuB">
        <img src="https://img.shields.io/discord/879125729936298015?logo=discord"
            alt="Visit the Hollow Knight Modding Discord server">
    </a>
    <a href="https://twitter.com/intent/follow?screen_name=JngoCreates">
        <img src="https://img.shields.io/twitter/follow/JngoCreates?style=social&logo=twitter"
             alt="Follow JngoCreates on Twitter">
    </a>

<p align='center'>
    <img src='./src-tauri/icons/icon.png' 
         alt='Butterfly logo' 
         width='192'
         height='192'/>
</p>

This is an installer and manager for Hollow Knight mods created using [**Tauri**](https://github.com/tauri-apps/tauri/) and [**VueJS**](https://github.com/vuejs/core/).

![Butterfly screenshot](/images/window.png)

## **Installation**
### Windows
1. Navigate to the [Releases](https://github.com/jngo102/Butterfly/releases) page.
2. Download the latest release for your platform. This will be `butterfly_{version}_x64_en-US.msi`.
3. Open up the location of the download in your file system and double click on the file that you downloaded.
4. Follow the steps in the wizard to install Butterfly. Most steps can be left with their default values.

### macOS
1. Navigate to the [Releases](https://github.com/jngo102/Butterfly/releases) page.
2. Download the latest release for your platform. This will be `butterfly_{version}_x64.dmg`.
3. Open up the location of the download in your file system and double click on the file that you downloaded.
4. Follow the steps in the wizard to install Butterfly.

### Linux
1. Navigate to the [Releases](https://github.com/jngo102/Butterfly/releases) page.
2. The file that you will download varies depending on which package manager you use. Refer to your distribution's manual for information on how to install packages for your system. For Ubuntu and Debian distributions, this is `butterfly_{version}_amd64.deb`.
3. Open up the location of the download in your file system.
4. Using your package manager, install the package that you downloaded.

## **Building from Source**
1. Ensure NodeJS is installed on your system. An installer for it can be found on their [website](https://nodejs.org/en/download/).
2. If it is not already installed, install the Yarn package manager using the command `npm install --global yarn`.
3. Clone this repository.
4. Run the command `yarn build`.
5. When yarn has finished building, run `yarn tauri build`.
6. The output executable can be found in `{repoPath}/src-tauri/target/{configuration}`, and the installer for it can be found in `{repoPath}/src-tauri/target/{configuration}/bundle/{installerType}`

## **Troubleshooting**
*Before [opening an issue on GitHub](https://github.com/jngo102/Butterfly/issues), make sure no one has had a similar problem that has been resolved.*

### The app opens, but the content shows an error page.
>Make sure you are connected to the Internet. Butterfly will not work without a working connection.

### The app never opens, or it opens and immediately closes.
>Delete the settings file and re-run the app. For Windows, this is at `%APPDATA%/Butterfly/Settings.json | C:/Users/{userName}/AppData/Roaming/Butterfly/Settings.json`. For macOS, this is at `$HOME/Library/Application Support/Butterfly/Settings.json | /Users/{userName}/Library/Application Support/Butterfly/Settings.json`. For Linux, this is `$HOME/.local/share/Butterfly/Settings.json | /home/{userName}/.local/share/Butterfly/Settings.json`."

### I cannot install or uninstall a mod.
>Make sure Hollow Knight is not running.

### I cannot enable/disable the Modding API
>[Verify the integrity](https://help.steampowered.com/en/faqs/view/0C48-FCBD-DA71-93EB) of your Hollow Knight files through Steam, then restart Butterfly.

For other issues, and if the app opens, you can report an issue by clicking on the `Report an issue/Suggest a feature` link in the app header. Otherwise, go to the [issues page](https://github.com/jngo102/Butterfly/issues?q=is%3Aissue), click the green `New issue` button, and select the `🦋 Bug Report` template using the green `Get started` button. Complete the fields to the best of your ability.

Additionally, you can enter the [Hollow Knight Modding Discord server](https://discord.gg/VDsg3HmWuB) and ask for help in the `#support` channel.

## **Contributing**
Pull requests are always welcome. Be sure to follow the [template](https://github.com/jngo102/Butterfly/blob/main/.github/PULL_REQUEST_TEMPLATE/pull_request.md).

Contributions to the app's localization are also desired. To add or edit a locale, navigate to `src/i18n.ts`, click on the pencil icon in the top right of the page (tooltip is "Edit this file"), and in the `messages` variable, translate the fields of the JSON object. Use the English fields as a baseline.

If you want to *add* a new locale, you will also need to edit `src/App.vue`. Look for the `languages` and `languagesMap` variables and add the new language's name to `languages` and a key-value pair of the new language name and new locale name in the `languagesMap` variable according to how the other locales are set up, i.e. don't forget the colon and comma.

Once you are finished, it is recommended that you enter a message describing what you changed at the bottom of the page, titled "Commit changes". Click the green "Propose changes" button once you are finished, then on the following pages, click the green "Create pull request" buttons to submit the changes.

## **License**
Butterfly is licensed under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html). This means if you fork and make modifications to this code, you must license your modification under the GPLv3 as well if you wish to distribute it.