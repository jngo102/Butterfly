<template>
  <nav
    id="nav-header"
    :class="'top-0 d-flex sticky-top nav navbar nav-justified justify-content-around ' + (theme == 'Dark' ? 'bg-dark navbar-dark' : 'bg-light navbar-light')" 
  >
    <span
      id="important-links"
      class="d-flex align-items-center justify-content-around"
      style="width: 100%"
    >
      <a
        :class="theme == 'Dark' ? 'link-light' : 'link-dark'"
        href="https://github.com/jngo102/Butterfly/blob/main/README.md"
      >
        {{ $t("message.readMe") }}
      </a>
      <a :class="theme == 'Dark' ? 'link-light' : 'link-dark'" href="https://github.com/jngo102/Butterfly/issues">
        {{ $t("message.report") }}
      </a>
      <a :class="theme == 'Dark' ? 'link-light' : 'link-dark'" href="https://github.com/jngo102/Butterfly">
        {{ $t("message.source") }}
      </a>
      <a :class="theme == 'Dark' ? 'link-light' : 'link-dark'" href="https://www.paypal.com/paypalme/jngo102">
        {{ $t("message.donate") }}
      </a>
      <span id="theme-toggle" :class="customTheme ? 'd-none' : ''">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          id="light-theme-icon"
          width="16"
          height="16"
          fill="#212529"
          class="bi bi-sun"
          viewBox="0 0 16 16"
        >
          <path
            d="M8 11a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm0 1a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8zm10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0zm-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707zM4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708z"
          />
        </svg>
        <div id="toggle-theme-form" class="form-check form-switch">
          <input
            id="toggle-theme-switch"
            class="form-check-input"
            type="checkbox"
            role="switch"
            :checked="theme == 'Dark'"
            @change="toggleTheme"
          />
          <svg
            xmlns="http://www.w3.org/2000/svg"
            id="dark-theme-icon"
            width="16"
            height="16"
            fill="#f8f9fa"
            class="bi bi-moon"
            viewBox="0 0 16 16"
          >
            <path
              d="M6 .278a.768.768 0 0 1 .08.858 7.208 7.208 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277.527 0 1.04-.055 1.533-.16a.787.787 0 0 1 .81.316.733.733 0 0 1-.031.893A8.349 8.349 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.752.752 0 0 1 6 .278zM4.858 1.311A7.269 7.269 0 0 0 1.025 7.71c0 4.02 3.279 7.276 7.319 7.276a7.316 7.316 0 0 0 5.205-2.162c-.337.042-.68.063-1.029.063-4.61 0-8.343-3.714-8.343-8.29 0-1.167.242-2.278.681-3.286z"
            />
          </svg>
        </div>
      </span>
    </span>
    <div id="visibility-tabs" class="d-flex">
      <ul
        id="filter-tabs"
        class="d-flex nav nav-fill nav-tabs"
        role="tablist"
      >
        <li class="nav-item" role="presentation">
          <button
            id="all-mods-tab"
            :class="'nav-link text-nowrap active ' + (theme == 'Dark' ? 'bg-dark text-light ' : 'bg-light text-dark') + (activeTab == 'All' ? ' active' : '')"
            href="#"
            @click="showAll"
            role="tab"
            :aria-selected="(activeTab == 'All' ? 'true' : 'false')"
          >
            {{ $t("message.all") }}
          </button>
        </li>
        <li class="nav-item" role="presentation">
          <button
            id="installed-mods-tab"
            :class="'nav-link text-nowrap ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark') + (activeTab == 'Installed' ? ' active' : '')"
            href="#"
            @click="showInstalled"
            role="tab"
            :aria-selected="(activeTab == 'Installed' ? 'true' : 'false')"
          >
            {{ $t("message.installed") }}
          </button>
        </li>
        <li class="nav-item" role="presentation">
          <button
            id="enabled-mods-tab"
            :class="'nav-link text-nowrap ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark') + (activeTab == 'Enabled' ? ' active' : '')"
            href="#"
            @click="showEnabled"
            role="tab"
            :aria-selected="(activeTab == 'Enabled' ? 'true' : 'false')"
          >
            {{ $t("message.enabled") }}
          </button>
        </li>
        <li class="dropdown">
          <a
            id="profiles-dropdown"
            :class="'nav-link text-nowrap dropdown-toggle ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark')"
            data-bs-toggle="dropdown"
            href="#"
            role="button"
            aria-expanded="false"
            data-bs-auto-close="false"
            @click="checkCurrentProfile"
          >
            {{ $t("message.profiles") }}
          </a>
          <ul id="profiles-dropdown-menu" :class="'p-1 dropdown-menu ' + (theme == 'Dark' ? 'dropdown-menu-dark' : '')">
            <ModProfile
              v-for="(profile, index) in profiles"
              :profileName="profile.Name"
              :profileMods="profile.Mods"
              :theme="theme"
              :key="index"
            />
            <button
              id="create-new-profile-button"
              :class="'btn btn-sm m-1 ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
              data-bs-toggle="modal"
              data-bs-target="#create-profile-modal"
            >
              {{ $t("message.createNewProfile") }}
            </button>
            <div class="btn-group d-flex justify-content-center mt-1">
              <button
                id="begin-export-profiles-button"
                :class="'btn btn-sm ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
                @click="beginExportProfiles"
              >
                {{ $t("message.exportProfiles") }}
              </button>
              <button
                id="import-profiles-button"
                :class="'btn btn-sm ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
                @click="importProfiles"
              >
                {{ $t("message.importProfiles") }}
              </button>
            </div>
            <div class="btn-group d-flex justify-content-center my-1">
              <button
                id="confirm-export-profiles-button"
                :class="'btn btn-small d-none ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
                @click="exportProfiles"
              >
                {{ $t("message.export") }}
              </button>
              <button
                id="cancel-export-profiles-button"
                :class="'btn btn-sm d-none ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
                @click="cancelExportProfiles"
              >
                {{ $t("message.cancel") }}
              </button>
            </div>
          </ul>
        </li>
        <li class="dropdown">
          <a
            id="import-save-dropdown"
            :class="'nav-link text-nowrap dropdown-toggle ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark')"
            data-bs-toggle="dropdown"
            href="#"
            role="button"
            aria-expanded="false"
          >
            {{ $t("message.importSave") }}
          </a>
          <ul id="import-save-dropdown-menu" :class="'p-1 dropdown-menu ' + (theme == 'Dark' ? 'dropdown-menu-dark' : '')">
            <a>{{ $t("message.chooseSaveSlot") }}</a>
            <a
              v-for="saveSlot in [1, 2, 3, 4]"
              :key="saveSlot"
              class="dropdown-item"
              @click="importSave(saveSlot)"
            >
              {{ saveSlot }}
            </a>
          </ul>
        </li>
        <li class="dropdown">
          <a
            id="languages-dropdown"
            :class="'nav-link text-nowrap dropdown-toggle ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark')"
            data-bs-toggle="dropdown"
            href="#"
            role="button"
            aria-expanded="false"
          >
            {{ $t("message.languages") }}
          </a>
          <ul id="languages-dropdown-menu" :class="'p-1 dropdown-menu ' + (theme == 'Dark' ? 'dropdown-menu-dark' : '')">
            <a
              v-for="(language, index) in languages"
              :key="index"
              class="dropdown-item"
              @click="changeLanguage(language)"
            >
              {{ language }}
            </a>
          </ul>
        </li>
      </ul>
    </div>
    <div id="action-buttons">
      <button
        id="toggle-api-button"
        :class="'btn btn-sm me-3 ' + (apiEnabled ? (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark') : (theme == 'Dark' ? 'btn-light' : 'btn-dark'))"
        @click="toggleApi"
      >
        {{ apiEnabled ? $t("message.disableApi") : $t("message.enableApi") }}
      </button>
      <div class="btn-group">
        <button
          id="open-mods-button"
          :class="'btn btn-sm ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
          @click="openMods"
        >
          {{ $t("message.openMods") }}
        </button>
        <button
          id="manually-install-mod-button"
          :class="'btn btn-sm ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
          @click="manuallyInstallMod"
        >
          {{ $t("message.manualInstall") }}
        </button>
      </div>
    </div>
    <div class="input-group input-group-sm px-1">
      <input
        type="search"
        id="mods-search"
        :class="'form-control input-sm ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark')"
        :placeholder="$t('message.searchMods')"
        @input="searchMods"
      />
    </div>
    <div
      id="current-download-progress"
      class="progress d-none"
      style="width: 100%"
    >
      <div
        id="current-download-progress-bar"
        class="progress-bar"
        role="progressbar"
        aria-valuenow="0"
        aria-valuemin="0"
        aria-valuemax="100"
      >
        0
      </div>
    </div>
    <div id="profile-creation-actions" class="btn-group">
      <button
        id="create-profile-button"
        :class="'btn d-none ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
        @click="createProfile"
      >
        {{ $t("message.createProfile") }}
      </button>
      <button
        id="cancel-create-profile-button"
        :class="'btn d-none ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
        @click="clearModProfileInputs"
      >
        {{ $t("message.cancel") }}
      </button>
    </div>
  </nav>
  <div
    id="create-profile-modal"
    class="modal fade bg-light"
    tabindex="-1"
    role="dialog"
    aria-hidden="true"
    aria-labelledby="create-new-profile-title"
  >
    <div class="modal-dialog" role="document">
      <div :class="'modal-content ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')">
        <div class="modal-header">
          <h5 id="create-new-profile-title" :class="'modal-title ' + (theme == 'Dark' ? 'text-light' : 'text-dark')">
            {{ $t("message.profileNamePrompt") }}
          </h5>
        </div>
        <div :class="'modal-body ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')">
          <div class="input-group input-group-sm">
            <input
              type="text"
              id="profile-name-input"
              :class="'form-control input-sm ' + (theme == 'Dark' ? 'bg-dark text-light' : 'bg-light text-dark')"
              :placeholder="$t('message.profileNamePlaceholder')"
            />
          </div>
        </div>
        <div :class="'modal-footer ' + (theme == 'Dark' ? 'bg-dark' : 'bg-light')">
          <button
            type="button"
            id="select-mods-button"
            :class="'btn ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
            data-bs-toggle="modal"
            data-bs-target="#create-profile-modal"
            @click="selectMods"
          >
            {{ $t("message.selectMods") }}
          </button>
          <button
            type="button"
            id="close-modal-button"
            :class="'btn ' + (theme == 'Dark' ? 'btn-outline-light' : 'btn-outline-dark')"
            data-bs-dismiss="modal"
            @click="clearModProfileInputs"
          >
            {{ $t("message.cancel") }}
          </button>
        </div>
      </div>
    </div>
  </div>
  <div
    id="mod-details-container"
    data-bs-spy="scroll"
    data-bs-target="#nav-header"
    data-bs-offset="0"
  >
    <ModDetails
      v-for="(manifest, index) in modLinks.Manifest"
      :modEnabled="manifest.Enabled"
      :modInstalled="manifest.Installed"
      :modName="manifest.Name"
      :modDescription="manifest.Description"
      :modVersion="manifest.Version"
      :modLink="manifest.Link.$value"
      :modNew="newMods.includes(manifest.Name)"
      :modOutdated="outdatedMods.includes(manifest.Name)"
      :sha256="manifest.Link.SHA256"
      :dependencies="manifest.Dependencies.Dependency"
      :theme="theme"
      :key="index"
    />
  </div>
</template>

<script lang='ts'>
import "bootstrap";
import { defineComponent } from "vue";
import ModDetails from "./components/ModDetails.vue";
import ModProfile from "./components/ModProfile.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { translate } from "./i18n";

export default defineComponent({
  name: "App",
  components: {
    ModDetails,
    ModProfile,
  },
  async mounted() {
    await this.reset();
  },
  data() {
    return {
      activeTab: "All",
      apiEnabled: false,
      languages: [
        "English",
        "中文",
        // "Deutsch",
        // "Español",
        "Français",
        // "русский",
      ],
      languagesMap: {
        English: "en",
        中文: "cn",
        // "Deutsch": 'de',
        // "Español": 'es',
        "Français": 'fr',
        // "русский": 'ru',
      },
      modLinks: {} as any,
      newMods: [] as string[],
      outdatedMods: [] as string[],
      profiles: [] as any[],
      currentProfile: "",
      theme: "Dark",
      customTheme: false,
    };
  },
  methods: {
    /**
     * Begin to select which profiles to export to a JSON file.
     */
    beginExportProfiles: async function (): Promise<void> {
      document
        .getElementById("begin-export-profiles-button")
        ?.classList.add("d-none");
      document
        .getElementById("import-profiles-button")
        ?.classList.add("d-none");
      document
        .querySelectorAll(".mod-profile-radio")
        .forEach((radio) => radio.classList.add("d-none"));
      document
        .querySelectorAll(".export-profile-checkbox")
        .forEach((checkbox) => {
          (checkbox as HTMLInputElement).checked = false;
          checkbox.classList.remove("d-none");
        });
      document
        .querySelectorAll(".mod-profile-radio")
        .forEach((radio) => radio.classList.add("d-none"));
      document
        .getElementById("confirm-export-profiles-button")
        ?.classList.remove("d-none");
      document
        .getElementById("cancel-export-profiles-button")
        ?.classList.remove("d-none");
    },

    /**
     * Fetch the mod links JSON from backend to build mod data.
     */
    buildModList: async function (): Promise<void> {
      await invoke("fetch_mod_list")
        .then((group: any): void => {
          group = group as [string, string[], string[]];
          const listString = group[0];
          this.newMods = group[1];
          this.outdatedMods = group[2];
          invoke("debug", {msg:"New mods: " + JSON.stringify(this.newMods) });
          invoke("debug", {msg:"Outdated mods: " + JSON.stringify(this.outdatedMods) });
          this.modLinks = JSON.parse(listString as string);
          invoke("debug", { msg: "listString: " + listString });
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Stop the process of exporting mod profiles to disk.
     */
    cancelExportProfiles: function (): void {
      document
        .getElementById("confirm-export-profiles-button")
        ?.classList.add("d-none");
      document
        .getElementById("cancel-export-profiles-button")
        ?.classList.add("d-none");
      document
        .querySelectorAll(".mod-profile-radio")
        .forEach((radio) => radio.classList.remove("d-none"));
      document
        .querySelectorAll(".export-profile-checkbox")
        .forEach((checkbox) => checkbox.classList.add("d-none"));
      document
        .getElementById("begin-export-profiles-button")
        ?.classList.remove("d-none");
      document
        .getElementById("import-profiles-button")
        ?.classList.remove("d-none");
    },

    /**
     * Change the app's current language.
     */
    changeLanguage: async function (language: string): Promise<void> {
      if (this.$root != undefined) {
        this.$root.$i18n.locale = (this.languagesMap as any)[
          language
        ] as string;
      }

      await invoke("set_language", { language: language });
    },

    /**
     * Check whether the Modding API has been installed.
     */
    checkApiInstalled: async function (): Promise<void> {
      await invoke("check_api_installed")
        .then((enabled) => {
          this.apiEnabled = enabled as boolean;
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Check the radio button of the current mod profile.
     */
    checkCurrentProfile: async function () {
      await invoke("fetch_current_profile")
        .then((currentProfile) => {
          this.currentProfile = currentProfile as string;
          const modProfiles = document.querySelectorAll(".mod-profile");
          modProfiles.forEach((profile) => {
            const label = profile.querySelector(
              ".mod-profile-label"
            ) as HTMLLabelElement;
            if (label.innerHTML == this.currentProfile) {
              const radio = profile.querySelector(
                ".mod-profile-radio"
              ) as HTMLInputElement;
              radio.checked = true;
            }
          });
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Clear the profile name input and checkboxes after cancelling creating a new profile.
     */
    clearModProfileInputs: function (): void {
      const profileNameInput = document.getElementById(
        "profile-name-input"
      ) as HTMLInputElement;
      profileNameInput.value = "";
      document.getElementById("create-profile-button")?.classList.add("d-none");
      document
        .getElementById("cancel-create-profile-button")
        ?.classList.add("d-none");
      const checkboxCols = document.querySelectorAll(".checkbox-col");
      checkboxCols.forEach((col) => {
        const checkbox = col.querySelector(
          ".profile-mod-checkbox"
        ) as HTMLInputElement;
        checkbox.checked = false;
        col.classList.add("d-none");
      });
      const modDetailsContainer = document.getElementById(
        "mod-details-container"
      ) as HTMLDivElement;
    },

    /**
     * Create a new mod profile
     */
    createProfile: async function (): Promise<void> {
      const profileNameInput = document.getElementById(
        "profile-name-input"
      ) as HTMLInputElement;
      const modDetailsRows = document.querySelectorAll(".mod-details-row");
      var modNames: Array<string> = [];
      modDetailsRows.forEach((row) => {
        if (
          (row.querySelector(".profile-mod-checkbox") as HTMLInputElement)
            .checked
        ) {
          const modName = row.querySelector(".mod-name")?.innerHTML;
          modNames.push(modName as string);
        }
      });
      let profileName = profileNameInput.value;
      await invoke("create_profile", {
        profileName: profileName,
        modNames: modNames,
      });
      this.profiles.push({ Name: profileName, Mods: modNames });
      modDetailsRows.forEach((row) => {
        (
          row.querySelector(".profile-mod-checkbox") as HTMLInputElement
        ).checked = false;
      });
      (
        document.getElementById("close-modal-button") as HTMLButtonElement
      ).click();
      this.clearModProfileInputs();
    },

    /**
     * Export a list of selected mod profiles in JSON format.
     */
    exportProfiles: async function (): Promise<void> {
      let profileNames: Array<string> = [];
      document.querySelectorAll(".mod-profile").forEach((profile) => {
        const exportCheckbox = profile.querySelector(
          ".export-profile-checkbox"
        ) as HTMLInputElement;
        const profileName = profile.querySelector(
          ".mod-profile-label"
        ) as HTMLLabelElement;
        if (exportCheckbox.checked) {
          profileNames.push(profileName.innerHTML);
        }
      });

      await invoke("export_profiles", { profileNames: profileNames }).
              then((success) => {
                if (success as boolean) {
                  this.cancelExportProfiles();
                }
              })
              .catch(error => invoke("debug", { msg: error }));
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
     * Get the saved application language from settings.
     */
    getLanguage: async function (): Promise<void> {
      await invoke("fetch_language")
        .then((language) => {
          if (this.$root != undefined) {
            this.$root.$i18n.locale = (this.languagesMap as any)[
              language as string
            ] as string;
          }
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Get all mod profiles from app settings.
     */
    getProfiles: async function (): Promise<void> {
      await invoke("fetch_profiles")
        .then((profileData: any) => {
          profileData = profileData as [string, string];
          const profilesString = profileData[0] as string;
          const currentProfile = profileData[1] as string;
          if ((profilesString as string) != "[]") {
            const profiles = JSON.parse(profilesString as string);
            this.profiles = profiles as Array<any>;
          }
          this.currentProfile = currentProfile;
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Get the app's theme from settings.
     */
    getTheme: async function (): Promise<void> {
      await invoke("fetch_theme_data")
        .then((group: any) => {
          group = group as [string, string, string];
          let theme = group[0];
          let themePath = group[1];
          let css = group[2];
          this.theme = theme;
          if (themePath != "") {
            this.customTheme = true;
            var style = document.querySelector("style") as HTMLStyleElement;
            if (style == null) {
              style = document.createElement("style");
            }
            style.innerText = css as string;
            document.head.appendChild(style);
          }
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Import a JSON file containing mod profile data.
     */
    importProfiles: async function (): Promise<void> {
      await invoke("import_profiles");
      await this.getProfiles();
    },

    /**
     * Import a Hollow Knight save file.
     */
    importSave: async function (saveSlot: number): Promise<void> {
      await invoke("import_save", { saveSlot: saveSlot });
    },

    /**
     * Manually install a mod from disk.
     */
    manuallyInstallMod: async function (): Promise<void> {
      await invoke("manually_install_mod").then((modName) => {
        if (modName == "") {
          return;
        }

        this.modLinks.Manifest.push({
          Name: modName,
          Description: "No description available.",
          Version: "Unknown",
          Link: "",
          Dependencies: [],
          Enabled: true,
          Installed: true,
        });
      });
    },

    /**
     * Open the local folder on the file system containing all installed mods.
     */
    openMods: function (): void {
      invoke("open_mods_folder");
    },
    
    /**
     * Replace all elements of a certain class with another class.
     */
    replaceClassAll: function (queryClass: string, replaceClass: string): void {
      document
        .querySelectorAll("." + queryClass)
        .forEach((element) =>
          element.classList.replace(queryClass, replaceClass)
        );
    },

    /**
     * Build all mod data again.
     */
    reset: async function (): Promise<void> {
      await this.getLanguage();
      await this.checkApiInstalled();
      await this.buildModList();
      await this.getProfiles();
      await this.getTheme();

      invoke("debug", {msg:"Mod Links: " + JSON.stringify(this.modLinks) });
    },

    /**
     * Filter the mod list based on search input.
     */
    searchMods: function (): void {
      const value = (
        document.getElementById("mods-search") as HTMLInputElement
      ).value?.toLowerCase() as string;
      invoke("debug", { msg: "Search input: " + value });
      const modDetails = document.querySelectorAll(".mod-details");
      modDetails.forEach((details) => {
        const modName = details
          .querySelector(".mod-name")
          ?.innerHTML.toLowerCase() as string;
        const modDesc = details
          .querySelector(".mod-description")
          ?.innerHTML.toLowerCase() as string;
        const enableDisableButton = details.querySelector(
          ".enable-disable-button"
        ) as HTMLButtonElement;
        const installUninstallButton = details.querySelector(
          ".install-uninstall-button"
        ) as HTMLButtonElement;
        if (
          (modName.includes(value) || modDesc.includes(value)) &&
          (this.activeTab == "All" ||
            (this.activeTab == "Enabled" &&
              !enableDisableButton.classList.contains("d-none") &&
              enableDisableButton.textContent ==
                translate("message.disable")) ||
            (this.activeTab == "Installed" &&
              installUninstallButton.textContent ==
                translate("message.uninstall")))
        ) {
          details.classList.remove("d-none");
        } else {
          details.classList.add("d-none");
        }
      });
    },

    /**
     * Begin to select mods to be included in the new profile.
     */
    selectMods: function (): void {
      document.getElementById("profiles-dropdown")?.click();
      const checkboxCols = document.querySelectorAll(".checkbox-col");
      checkboxCols.forEach((col) => col.classList.remove("d-none"));
      document
        .getElementById("create-profile-button")
        ?.classList.remove("d-none");
      document
        .getElementById("cancel-create-profile-button")
        ?.classList.remove("d-none");
    },

    /**
     * Set the global theme to dark mode.
     */
    setDarkTheme: async function (): Promise<void> {
      this.theme = "Dark";
      await invoke("set_theme", { themeName: this.theme });
      document
        .querySelector("body")
        ?.setAttribute("style", "background-color:#212529");
    },

    /**
     * Set the global theme to light mode.
     */
    setLightTheme: async function (): Promise<void> {
      this.theme = "Light";
      await invoke("set_theme", { themeName: this.theme });
      document
        .querySelector("body")
        ?.setAttribute("style", "background-color:#f8f9fa");
    },

    /**
     * Activate the "All" tab.
     */
    showAll: function (): void {
      this.activeTab = "All";
      this.searchMods();
    },

    /**
     * Activate the "Enabled" tab.
     */
    showEnabled: function (): void {
      this.activeTab = "Enabled";
      this.searchMods();
    },

    /**
     * Activate the "Installed" tab.
     */
    showInstalled: function (): void {
      this.activeTab = "Installed";
      this.searchMods();
    },

    /**
     * Toggle the Modding API.
     */
    toggleApi: async function (): Promise<void> {
      await invoke("toggle_api")
        .then((enabled) => {
          this.apiEnabled = enabled as boolean;
        })
        .catch((error) => invoke("debug", { msg: error }));
    },

    /**
     * Toggle the global theme between light and dark.
     */
    toggleTheme: function (): void {
      if (this.theme == "Dark") {
        this.setLightTheme();
      } else if (this.theme == "Light") {
        this.setDarkTheme();
      }
    },
  },
});
</script>

<style>
@import "~bootstrap/dist/css/bootstrap.css";

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: left;
}

body {
  background-color: #f8f9fa;
}

.accordion-button::after {
  content: none;
}

#theme-toggle * {
  display: inline-block;
}

#toggle-theme-form {
  padding-left: 3em;
}
</style>
