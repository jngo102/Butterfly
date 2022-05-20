<template>
  <li
    :id="fitTextToAttribute(profileName) + '-check'"
    class="dropdown-item form-check mod-profile"
  >
    <div id="profile-inputs" class="ms-1">
      <input
        :id="'export-profile-' + fitTextToAttribute(profileName)"
        class="form-check-input export-profile-checkbox d-none"
        type="checkbox"
      />
      <input
        :id="fitTextToAttribute(profileName) + '-radio'"
        class="form-check-input mod-profile-radio"
        type="radio"
        name="mod-profiles"
        @click="changeProfile"
      />
    </div>
    <label
      :class="'form-check-label mod-profile-label m-1 ' + (theme == 'Dark' ? 'text-light' : 'text-dark')"
      :for="fitTextToAttribute(profileName) + '-radio'"
    >
      {{ profileName }}
    </label>
    <button
      :class="'btn btn-sm delete-profile-button ms-3 me-1' + (theme == 'Dark' ? 'btn-outline-light' : 'bg-outline-dark')"
      @click="deleteProfile"
    >
      {{ $t("message.delete") }}
    </button>
    <input
      v-for="(mod, index) in profileMods"
      :id="profileName + '-' + mod"
      class="profile-mod-name"
      type="hidden"
      :value="mod"
      :key="index"
    />
  </li>
</template>

<script lang='ts'>
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { translate } from "../i18n";

export default defineComponent({
  name: "ModProfile",
  props: {
    profileName: String,
    profileMods: Array,
    theme: String,
  },
  methods: {
    /**
     * Change your current profile.
     */
    changeProfile: async function (): Promise<void> {
      await invoke("set_profile", { profileName: this.profileName });
      const li = document.getElementById(
        this.fitTextToAttribute(this.profileName as string) + "-check"
      ) as HTMLLIElement;
      const radioButton = li.querySelector(
        ".form-check-input"
      ) as HTMLInputElement;
      const checked = radioButton.value;
      if (checked != "on") return;

      const modNameInputs = li.querySelectorAll(".profile-mod-name");
      var modNames: Array<string> = [];
      var profileModDeps: Array<string> = [];
      modNameInputs?.forEach((input) => {
        const inputValue = (input as HTMLInputElement).value;
        modNames.push(inputValue);
        const modLinkElement = document.getElementById(
          "mod-link-" + this.fitTextToAttribute(inputValue)
        ) as HTMLInputElement;
        const modVersionElement = document.getElementById(
          "mod-version-" + this.fitTextToAttribute(inputValue)
        ) as HTMLInputElement;
        const modHashElement = document.getElementById(
          "mod-hash-" + this.fitTextToAttribute(inputValue)
        ) as HTMLInputElement;
        this.installMod(
          inputValue,
          modVersionElement.innerHTML,
          modHashElement.value,
          modLinkElement.value
        );
        const modDeps = document.querySelectorAll(
          "#dependencies-" + inputValue + " ul li"
        );
        modDeps.forEach((dep) =>
          profileModDeps.push(dep.textContent as string)
        );
      });
      const allModDetails = document.querySelectorAll(".mod-details");
      allModDetails.forEach((details) => {
        const modName = details.querySelector(".mod-name")
          ?.textContent as string;
        const enableDisableButton = details.querySelector(
          ".enable-disable-button"
        ) as HTMLButtonElement;
        if (!modNames.includes(modName) && !profileModDeps.includes(modName)) {
          invoke("disable_mod", { modName: modName });
          enableDisableButton.textContent = translate("message.enable");
          enableDisableButton.classList.replace("btn-outline-dark", "btn-dark");
          enableDisableButton.classList.replace("btn-outline-light", "btn-light");
          if (
            document
              .getElementById("enabled-mods-tab")
              ?.classList.contains("active")
          ) {
            details.classList.add("d-none");
          }
        }
      });
    },

    /**
     * Delete a profile.
     */
    deleteProfile: async function (): Promise<void> {
      await invoke("delete_profile", { profileName: this.profileName });
      let profileElement = document.getElementById(
        this.fitTextToAttribute(this.profileName as string) + "-check"
      ) as HTMLLIElement;
      profileElement.remove();
    },

    /**
     * Modifies text so that it may be used in an attribute, i.e. removing spaces
     * and non-alphanumeric characters.
     * @param {string} text The text to be modified
     * @return {string}     The modified text
     */
    fitTextToAttribute: function (text: string): string {
      return text.replace(/\W+/g, "");
    },

    /**
     * Install a mod.
     * @param {string} modName The name of the mod to be installed
     * @param {string} modVersion The mod's version to be installed
     * @param {string} modHash The SHA256 hash of the mod to be installed
     * @param {string} modLink The link to the download of the mod to be installed
     */
    installMod: async function (
      modName: string,
      modVersion: string,
      modHash: string,
      modLink: string
    ): Promise<void> {
      invoke("install_mod", {
        modName: modName,
        modVersion: modVersion,
        modHash: modHash,
        modLink: modLink,
      });
      const progressElement = document.getElementById(
        "current-download-progress"
      ) as HTMLDivElement;
      const progressBar = document.getElementById(
        "current-download-progress-bar"
      ) as HTMLDivElement;
      progressBar.ariaValueNow = "0";
      var buttons = document.querySelectorAll(
        ".install-uninstall-button, .enable-disable-button, .reset-button"
      );
      buttons.forEach((button) => button.setAttribute("disabled", "true"));
      var current_download_progress = 0;
      progressElement.classList.remove("d-none");
      while (current_download_progress < 100) {
        await invoke("fetch_current_download_progress")
          .then((progress) => {
            progressBar.style.width = (progress as string) + "%";
            progressBar.ariaValueNow = progress as string;
            progressBar.innerHTML = (progress as string) + "%";
            current_download_progress = progress as number;
          })
          .catch((error) => invoke("debug", { msg: error }));
      }
      progressElement.classList.add("d-none");
      buttons.forEach((button) => button.removeAttribute("disabled"));
      const installUninstallButton = document.getElementById(
        "install-uninstall-button-" + this.fitTextToAttribute(modName)
      ) as HTMLButtonElement;
      const enableDisableButton = document.getElementById(
        "enable-disable-button-" + this.fitTextToAttribute(modName)
      ) as HTMLButtonElement;
      const resetButton = document.getElementById(
        "reset-button-" + this.fitTextToAttribute(modName)
      ) as HTMLButtonElement;
      const readmeButton = document.getElementById(
        "readme-button-" + this.fitTextToAttribute(modName)
      ) as HTMLButtonElement;
      enableDisableButton.classList.remove("d-none");
      enableDisableButton.textContent = translate("message.disable");
      enableDisableButton.classList.replace("btn-dark", "btn-outline-dark");
      enableDisableButton.classList.replace("btn-light", "btn-outline-light");
      installUninstallButton.textContent = translate("message.uninstall");
      installUninstallButton.classList.replace("btn-dark", "btn-outline-dark");
      installUninstallButton.classList.replace("btn-light", "btn-outline-light");
      resetButton.classList.remove("d-none");
      readmeButton.classList.remove("d-none");
      const modDetails = document.getElementById(
        "mod-details-" + this.fitTextToAttribute(modName)
      );
      const value = (
        document.getElementById("mods-search") as HTMLInputElement
      ).value?.toLowerCase() as string;
      if (modName.includes(value)) {
        modDetails?.classList.remove("d-none");
      }

      // Install dependencies
      const dependencyElement = document.getElementById(
        "dependency-" + this.fitTextToAttribute(modName)
      ) as HTMLUListElement;
      const dependencies = dependencyElement.querySelectorAll("li");
      dependencies.forEach((dep) => {
        invoke("debug", {
          msg:
            "Installing dependency of {" +
            modName +
            "}: {" +
            dep.innerText +
            "}",
        });
        const modLinkElement = document.getElementById(
          "mod-link-" + this.fitTextToAttribute(dep.innerText)
        ) as HTMLInputElement;
        const modVersionElement = document.getElementById(
          "mod-version-" + this.fitTextToAttribute(dep.innerText)
        ) as HTMLParagraphElement;
        const modHashElement = document.getElementById(
          "mod-hash-" + this.fitTextToAttribute(dep.innerText)
        ) as HTMLInputElement;
        this.installMod(
          dep.innerText,
          modVersionElement.innerHTML,
          modHashElement.value,
          modLinkElement.value
        );
      });
    },
  },
});
</script>