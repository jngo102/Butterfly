<template>
  <div
    :ref="'ref-' + fitTextToAttribute(modName)"
    :class="'accordion accordion-flush mod-details ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')"
    :id="'mod-details-' + fitTextToAttribute(modName)"
  >
    <div
      :class="'accordion-item ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')"
      :id="'mod-main-' + fitTextToAttribute(modName)"
    >
      <div
        :class="'accordion-header container-fluid form-check ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')"
        :id="'mod-header-' + fitTextToAttribute(modName)"
      >
        <div class="container-fluid row mod-details-row">
          <button
            :class="'d-flex container-fluid row col accordion-button ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')"
            data-bs-toggle="collapse"
            :data-bs-target="
              '#collapsed-details-' + fitTextToAttribute(modName)
            "
            aria-expanded="false"
            :aria-controls="'collapsed-details-' + fitTextToAttribute(modName)"
          >
            <div class="checkbox-col col d-none">
              <input
                :id="'profile-checkbox-' + fitTextToAttribute(modName)"
                class="profile-mod-checkbox form-check-input"
                type="checkbox"
              />
            </div>
            <p :class="'flex-shrink-1 col ' + (theme == 'Dark' ? 'text-light' : 'text-dark') + (isNew ? '' : ' d-none')">
              <b class="new-mod">{{ $t("message.newMod") }}</b>
            </p>
            <p :class="'flex-shrink-1 col ' + (theme == 'Dark' ? 'text-light' : 'text-dark')">
              <b class="mod-name">{{ modName }}</b>
            </p>
            <input
              :id="'mod-link-' + fitTextToAttribute(modName)"
              class="mod-link"
              type="hidden"
              :value="modLink"
            />
            <input
              :id="'mod-hash-' + fitTextToAttribute(modName)"
              class="mod-hash"
              type="hidden"
              :value="sha256"
            />
            <p
              :id="'mod-version-' + fitTextToAttribute(modName)"
              :class="'flex-grow-1 col mod-version ' + (theme == 'Dark' ? 'text-light' : 'text-dark')"
            >
              {{ modVersion }}
            </p>
          </button>
          <div
            id="actions-button-group"
            class="btn-group col justify-contents-center"
            role="group"
            aria-label="Mod actions button group"
          >
            <button
              :class="
                'btn ' +
                (installed ? 
                  (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark') : 
                  (theme == 'Dark' ? 'btn-light' : 'btn-dark')) +
                ' align-self-center install-uninstall-button'
              "
              :id="'install-uninstall-button-' + fitTextToAttribute(modName)"
              @click="installOrUninstallMod"
            >
              {{
                installed ? $t("message.uninstall") : $t("message.install")
              }}
            </button>
            <button
              :class="
                'btn ' +
                (enabled ? 
                  (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark') : 
                  (theme == 'Dark' ? 'btn-light' : 'btn-dark')) +
                ' align-self-center enable-disable-button ' +
                (installed ? '' : ' d-none')
              "
              :id="'enable-disable-button-' + fitTextToAttribute(modName)"
              @click="enableOrDisableMod"
            >
              {{ enabled ? $t("message.disable") : $t("message.enable") }}
            </button>
            <button
              :id="'update-button-' + fitTextToAttribute(modName)"
              :class="'btn align-self-center update-button ' + 
               (theme == 'Dark' ? 'btn-light' : 'btn-dark') +
               (isOutdated ? '' : ' d-none')"
              @click="updateMod"
            >
              {{ $t("message.update") }}
            </button>
            <button
              :id="'reset-button-' + fitTextToAttribute(modName)"
              :class="
                'btn col align-self-center reset-button ' +
                (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark') +
                (installed ? '' : ' d-none')
              "
              @click="resetSettings"
            >
              {{ $t("message.reset") }}
            </button>
          </div>
        </div>
      </div>
      <div
        :class="'accordion-collapse collapse ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')"
        :id="'collapsed-details-' + fitTextToAttribute(modName)"
        :aria-labelledby="'mod-header-' + fitTextToAttribute(modName)"
        :data-bs-parent="'#mod-details-' + fitTextToAttribute(modName)"
      >
        <div :class="'accordion-body ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')">
          <p :class="'mod-description ' + (theme == 'Dark' ? 'text-light' : 'text-dark')">{{ modDescription }}</p>
          <div :id="'dependencies-' + modName" class="dependencies">
            <p :class="(theme == 'Dark' ? 'text-light' : 'text-dark')">
              <b>{{ $t("message.dependencies") }}</b>
            </p>
            <ul :id="'dependency-' + fitTextToAttribute(modName)">
              <li
                :class="(theme == 'Dark' ? 'text-light' : 'text-dark')"
                v-for="dependency in dependencies"
                :key="dependency"
              >
                {{ dependency }}
              </li>
            </ul>
            <button
              :id="'readme-button-' + fitTextToAttribute(modName)"
              :class="'btn col align-self-center readme-button ' +
              (theme == 'Dark' ? 'btn-outline-light ' : 'btn-outline-dark ') +
              (installed ? '' : 'd-none')
              "
              @click="openModReadMe"
            >
              {{ $t("message.modReadMe")}}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang='ts'>
import "bootstrap";
import { defineComponent, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { translate } from "../i18n";

export default defineComponent({
  name: "ModDetails",
  props: {
    modEnabled: Boolean,
    modInstalled: Boolean,
    modName: String,
    modDescription: String,
    modVersion: String,
    modLink: String,
    modNew: Boolean,
    modOutdated: Boolean,
    sha256: String,
    dependencies: Array,
    theme: String,
  },
  data() {
    return {
      enabled: this.modEnabled,
      installed: this.modInstalled,
      isNew: this.modNew,
      isOutdated: this.modOutdated,
    };
  },
  methods: {
    /**
     * Either enables or disables a mod depending on the mod's current enabled status.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    enableOrDisableMod: async function (event: MouseEvent): Promise<void> {
      const enableDisableButton = document.getElementById(
        "enable-disable-button-" +
          this.fitTextToAttribute(this.modName as string)
      ) as HTMLButtonElement;
      if (event.target != enableDisableButton) return;
      await invoke(this.enabled ? "disable_mod" : "enable_mod", {
        modName: this.modName,
      });
      this.enabled = !this.enabled;
      enableDisableButton.textContent = this.enabled
        ? translate("message.disable")
        : translate("message.enable");
      const modDetails = document.getElementById(
        "mod-details-" + this.fitTextToAttribute(this.modName as string)
      ) as HTMLDivElement;
      if (enableDisableButton.textContent == translate("message.enable")) {
        enableDisableButton.classList.replace("btn-outline-dark", "btn-dark");
        enableDisableButton.classList.replace("btn-outline-light", "btn-light");
        if (
          document
            .getElementById("enabled-mods-tab")
            ?.classList.contains("active")
        ) {
          modDetails.classList.add("d-none");
        }
      } else if (
        enableDisableButton.textContent == translate("message.disable")
      ) {
        enableDisableButton.classList.replace("btn-dark", "btn-outline-dark");
        enableDisableButton.classList.replace("btn-light", "btn-outline-light");
        modDetails.classList.remove("d-none");
      }
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
        ".install-uninstall-button, .enable-disable-button"
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

      this.installed = true;
    },

    /**
     * Either installs or uninstalls a mod depending on its current installation status.
     * Also automatically installs the mod's dependencies.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    installOrUninstallMod: async function (event: MouseEvent): Promise<void> {
      const installUninstallButton = document.getElementById(
        "install-uninstall-button-" +
          this.fitTextToAttribute(this.modName as string)
      ) as HTMLButtonElement;
      if (event.target != installUninstallButton) return;
      const enableDisableButton = document.getElementById(
        "enable-disable-button-" +
          this.fitTextToAttribute(this.modName as string)
      ) as HTMLButtonElement;
      const resetButton = document.getElementById(
        "reset-button-" + this.fitTextToAttribute(this.modName as string)
      ) as HTMLButtonElement;

      const readmeButton = document.getElementById(
        "readme-button-" + this.fitTextToAttribute(this.modName as string)
      ) as HTMLButtonElement;
      if (this.installed) {
        await invoke("uninstall_mod", { modName: this.modName });
        enableDisableButton.classList.add("d-none");
        resetButton.classList.add("d-none");
        readmeButton.classList.add("d-none");
        if (this.modLink == null) {
          const modDetails = document.getElementById(
            "mod-details-" + this.fitTextToAttribute(this.modName as string)
          );
          modDetails?.remove();
        }

        if (
          document
            .getElementById("installed-mods-tab")
            ?.classList.contains("active") ||
          document
            .getElementById("enabled-mods-tab")
            ?.classList.contains("active")
        ) {
          const modDetails = document.getElementById(
            "mod-details-" + this.fitTextToAttribute(this.modName as string)
          );
          modDetails?.classList.add("d-none");
        }
        this.installed = false;
      } else {
        this.installMod(
          this.modName as string,
          this.modVersion as string,
          this.sha256 as string,
          this.modLink as string
        );
      }

      this.enabled = true;
    },

    /**
     * Open a mod's read me file.
     */
    openModReadMe: async function (): Promise<void> {
      const modDetails = document.getElementById(
        "mod-details-" + this.fitTextToAttribute(this.modName as string)
      ) as HTMLDivElement;
      const modName = modDetails.querySelector(".mod-name")
        ?.innerHTML as string;
      await invoke("open_mod_read_me", { modName: modName });
    },

    /**
     * Reset a mod's global settings.
     */
    resetSettings: async function (): Promise<void> {
      const modDetails = document.getElementById(
        "mod-details-" + this.fitTextToAttribute(this.modName as string)
      ) as HTMLDivElement;
      const modName = modDetails.querySelector(".mod-name")
        ?.innerHTML as string;
      await invoke("reset_settings", { modName: modName });
    },

    /**
     * Update an installed mod to the most recent version on modlinks.
     */
    updateMod: function (): void {
      const updateModButton = document.getElementById(
        "update-button-" + this.fitTextToAttribute(this.modName as string)
      ) as HTMLButtonElement;
      updateModButton.classList.add("d-none");
      this.installMod(
        this.modName as string,
        this.modVersion as string,
        this.sha256 as string,
        this.modLink as string
      );
    },
  },
});
</script>

<style scoped>
.accordion-button {
  text-align: center;
}
</style>