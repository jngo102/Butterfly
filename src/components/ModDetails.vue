<template>
  <div
    class="accordion accordion-flush mod-details bg-light"
    :id="'mod-details-' + fitTextToAttribute(mod.name)"
  >
    <div
      class="accordion-item bg-light"
      :id="'mod-main-' + fitTextToAttribute(mod.name)"
    >
      <div
        class="accordion-header form-check bg-light"
        :id="'mod-header-' + fitTextToAttribute(mod.name)"
      >
        <div class="row mod-details-row">
          <div
            class="checkbox-col col d-none"
          >
            <input
              :id="'profile-checkbox-' + fitTextToAttribute(mod.name)"
              class="profile-mod-checkbox form-check-input"
              type="checkbox"
            />
          </div>
          <button
            class="d-flex col accordion-button bg-light"
            data-bs-toggle="collapse"
            :data-bs-target="
              '#collapsed-details-' + fitTextToAttribute(mod.name)
            "
            aria-expanded="false"
            :aria-controls="'collapsed-details-' + fitTextToAttribute(mod.name)"
          >
            <p class="text-dark">
              <b class="mod-name">{{ mod.name }}</b>
            </p>
            <input
              :id="'mod-link-' + fitTextToAttribute(mod.name)"
              class="mod-link"
              type="hidden"
              :value="modLink"
            />
            <input
              :id="'mod-hash-' + fitTextToAttribute(mod.name)"
              class="mod-hash"
              type="hidden"
              :value="sha256"
            />
            <p
              :id="'mod-version-' + fitTextToAttribute(mod.name)"
              class="col mod-version text-dark"
            >
              {{ modVersion }}
            </p>
          </button>
          <div
            id="actions-button-group"
            class="btn-group col"
            role="group"
            aria-label="Mod actions button group"
          >
            <button
              :class="
                'btn ' +
                (mod.installed ? 'btn-outline-dark' : 'btn-dark') +
                ' col align-self-center install-uninstall-button'
              "
              :id="'install-uninstall-button-' + fitTextToAttribute(mod.name)"
              @click="installOrUninstallMod"
            >
              {{ mod.installed ? $t("message.uninstall") : $t("message.install") }}
            </button>
            <button
              :class="
                'btn ' +
                (mod.enabled ? 'btn-outline-dark' : 'btn-dark') +
                ' col align-self-center enable-disable-button ' +
                (mod.installed ? '' : 'd-none')
              "
              :id="'enable-disable-button-' + fitTextToAttribute(mod.name)"
              @click="enableOrDisableMod"
            >
              {{ mod.enabled ? $t("message.disable") : $t("message.enable") }}
            </button>
            <button
              :id="'update-button-' + fitTextToAttribute(mod.name)"
              class="btn btn-dark col align-self-center d-none update-button"
              @click="updateMod"
            >
              {{ $t("message.update") }}
            </button>
            <button
              :id="'reset-button-' + fitTextToAttribute(mod.name)"
              :class="
                'btn btn-outline-dark col align-self-center reset-button ' +
                (mod.installed ? '' : 'd-none')
              "
              @click="resetSettings"
            >
              {{ $t("message.reset") }}
            </button>
          </div>
        </div>
      </div>
      <div
        class="accordion-collapse collapse bg-light"
        :id="'collapsed-details-' + fitTextToAttribute(mod.name)"
        :aria-labelledby="'mod-header-' + fitTextToAttribute(mod.name)"
        :data-bs-parent="'#mod-details-' + fitTextToAttribute(mod.name)"
      >
        <div class="accordion-body bg-light">
          <p class="mod-description text-dark">{{ modDescription }}</p>
          <div :id="'dependencies-' + mod.name" class="dependencies">
            <p class="text-dark"><b>{{ $t("message.dependencies") }}</b></p>
            <ul :id="'dependency-' + fitTextToAttribute(mod.name)">
              <li
                class="text-dark"
                v-for="dependency in dependencies"
                :key="dependency"
              >
                {{ dependency }}
              </li>
            </ul>
            <button
              :id="'readme-button-' + fitTextToAttribute(mod.name)"
              :class="'btn btn-outline-dark col align-self-center readme-button ' +
              (mod.installed ? '' : 'd-none')
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
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { translate } from "../i18n";

export class ModItem {
  name: string;
  installed: boolean;
  enabled: boolean;

  constructor(name: string, installed: boolean, enabled: boolean) {
    this.name = name;
    this.installed = installed;
    this.enabled = enabled;
  }
}

export default defineComponent({
  name: "ModDetails",
  props: {
    mod: ModItem,
    modDescription: String,
    modVersion: String,
    modLink: String,
    sha256: String,
    dependencies: Array,
  },
  methods: {
    /**
     * Either enables or disables a mod depending on the mod's current enabled status.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    enableOrDisableMod: async function (event: MouseEvent): Promise<void> {
      const enableDisableButton = document.getElementById(
        "enable-disable-button-" +
          this.fitTextToAttribute((this.mod as ModItem).name)
      ) as HTMLButtonElement;
      if (event.target != enableDisableButton) return;
      await invoke(this.mod?.enabled ? "disable_mod" : "enable_mod", {
        modName: this.mod?.name,
      });
      (this.mod as ModItem).enabled = !this.mod?.enabled;
      enableDisableButton.textContent = this.mod?.enabled
        ? translate("message.disable")
        : translate("message.enable")
      const modDetails = document.getElementById(
        "mod-details-" + this.fitTextToAttribute(this.mod?.name as string)
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
      } else if (enableDisableButton.textContent == translate("message.disable")) {
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
          .catch((e) => invoke("debug", { msg: e }));
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
        "reset-button-" + this.fitTextToAttribute((this.mod as ModItem).name)
      ) as HTMLButtonElement;
      const readmeButton = document.getElementById(
        "readme-button-" + this.fitTextToAttribute((this.mod as ModItem).name)
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

    /**
     * Either installs or uninstalls a mod depending on its current installation status.
     * Also automatically installs the mod's dependencies.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    installOrUninstallMod: async function (event: MouseEvent): Promise<void> {
      const installUninstallButton = document.getElementById(
        "install-uninstall-button-" +
          this.fitTextToAttribute((this.mod as ModItem).name)
      ) as HTMLButtonElement;
      if (event.target != installUninstallButton) return;
      const enableDisableButton = document.getElementById(
        "enable-disable-button-" +
          this.fitTextToAttribute((this.mod as ModItem).name)
      ) as HTMLButtonElement;
      const resetButton = document.getElementById(
        "reset-button-" + this.fitTextToAttribute((this.mod as ModItem).name)
      ) as HTMLButtonElement;
      const readmeButton = document.getElementById(
        "readme-button-" + this.fitTextToAttribute((this.mod as ModItem).name)
      ) as HTMLButtonElement;
      if (installUninstallButton.textContent == translate("message.uninstall")) {
        await invoke("uninstall_mod", { modName: this.mod?.name });
        enableDisableButton.classList.add("d-none");
        resetButton.classList.add("d-none");
        readmeButton.classList.add("d-none");
        if (this.modLink == null) {
          const modDetails = document.getElementById(
            "mod-details-" + this.fitTextToAttribute(this.mod?.name as string)
          );
          modDetails?.remove();
        } else {
          installUninstallButton.textContent = translate("message.install");
          enableDisableButton.classList.replace("btn-outline-dark", "btn-dark");
          enableDisableButton.classList.replace("btn-outline-light", "btn-light");
          installUninstallButton.classList.replace("btn-outline-dark", "btn-dark");
          installUninstallButton.classList.replace("btn-outline-light", "btn-light");
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
            "mod-details-" + this.fitTextToAttribute(this.mod?.name as string)
          );
          modDetails?.classList.add("d-none");
        }
        (this.mod as ModItem).installed = true;
      } else {
        this.installMod(
          this.mod?.name as string,
          this.modVersion as string,
          this.sha256 as string,
          this.modLink as string
        );
        (this.mod as ModItem).installed = false;
      }

      (this.mod as ModItem).enabled = true;
    },

    /**
     * Open a mod's read me file.
     */
    openModReadMe: async function (): Promise<void> {
      const modDetails = document.getElementById(
        "mod-details-" + this.fitTextToAttribute(this.mod?.name as string)
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
        "mod-details-" + this.fitTextToAttribute(this.mod?.name as string)
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
        "update-button-" + this.fitTextToAttribute(this.mod?.name as string)
      ) as HTMLButtonElement;
      updateModButton.classList.add("d-none");
      this.installMod(
        this.mod?.name as string,
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