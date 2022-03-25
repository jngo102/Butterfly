# Butterfly
![Butterfly logo](/src-tauri/icons/icon.png)
This is an installer and manager for Hollow Knight mods created using **Tauri** (https://github.com/tauri-apps/tauri/) and **VueJS** (https://github.com/vuejs/core/).

![Butterfly screenshot](/screenshots/window.png)

## Installation
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

## Building from Source
1. Ensure NodeJS is installed on your system. An installer for it can be found on their [website](https://nodejs.org/en/download/).
2. If it is not already installed, install the Yarn package manager using `npm install --global yarn`.
3. Clone this repository.
4. Run the command `yarn build`.
5. When yarn has finished building, run `yarn tauri build`.
6. The output executable can be found in `{repoPath}/src-tauri/target/{configuration}`, and the installer for it can be found in `{repoPath}/src-tauri/target/{configuration}/bundle/{installerType}`

## Troubleshooting
**Before opening an issue on GitHub, make sure no one has had a similar problem that has been resolved.**

### The app opens, but the content shows an error page.
Make sure you are connected to the Internet. Butterfly will not work without a working connection.


### The app never opens, or it opens and immediately closes.
Delete the settings file and re-run the app. For Windows, this is at `%APPDATA%/Butterfly/Settings.json | C:/Users/{userName}/AppData/Roaming/Butterfly/Settings.json`. For macOS, this is at `$HOME/Library/Application Support/Butterfly/Settings.json | /Users/{userName}/Library/Application Support/Butterfly/Settings.json`. For Linux, this is `$HOME/.local/share/Butterfly/Settings.json | /home/{userName}/.local/share/Butterfly/Settings.json`."

### I cannot install or uninstall a mod.
Make sure Hollow Knight is not running.

### I cannot enable/disable the Modding API
[Verify the integrity](https://help.steampowered.com/en/faqs/view/0C48-FCBD-DA71-93EB) of your Hollow Knight files through Steam, then restart Butterfly.

For other issues, and if the app opens, you can report an issue by clicking on the `Report an issue/Suggest a feature` link in the app header. Otherwise, go to the [issues page](https://github.com/jngo102/Butterfly/issues?q=is%3Aissue), click the green `New issue` button, and select the `🦋 Bug Report` template using the green `Get started` button. Complete the fields to the best of your ability.

Additionally, you can enter the [Hollow Knight Modding Discord server](https://discord.gg/VDsg3HmWuB) and ask for help in the `#support` channel.

## Pull Requests
Pull requests are always welcome. Be sure to follow the template located in `.github/PULL_REQUEST_TEMPLATE/pull_request.md`.

## License
Butterfly is licensed under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html). This means if you fork and make modifications to this code, you must license your modification under the GPLv3 as well if you want to distribute it.