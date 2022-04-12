<template>
  <nav
    id="nav-header"
    class="
      d-flex
      nav
      navbar navbar-dark
      nav-justified
      justify-content-around
      fixed-top
      bg-dark
    "
  >
    <span
      id="important-links"
      class="d-flex align-items-center justify-content-around"
      style="width: 100%"
    >
      <a
        class="link-info px-5"
        href="https://github.com/jngo102/Butterfly/blob/main/README.md"
        @click="openReadme()"
      >
        Read Me
      </a>
      <a
        class="link-info px-5"
        href="https://github.com/jngo102/Butterfly/issues"
      >
        Report a Bug/Suggest a Feature
      </a>
      <a class="link-info px-5" href="https://github.com/jngo102/Butterfly">
        Source Code
      </a>
      <a class="link-info px-5" href="https://www.paypal.com/paypalme/jngo102">
        Buy me a coffee
      </a>
    </span>
    <div id="visibility-tabs">
      <ul
        id="filter-tabs"
        class="nav nav-tabs nav-justified bg-dark justify-content-center"
        role="tablist"
      >
        <li class="nav-item" role="presentation">
          <button
            id="all-mods-tab"
            class="nav-link text-white active"
            aria-current="page"
            href="#"
            @click="showAll"
            role="tab"
            aria-selected="true"
          >
            All
          </button>
        </li>
        <li class="nav-item" role="presentation">
          <button
            id="installed-mods-tab"
            class="nav-link text-white"
            href="#"
            @click="showInstalled"
            role="tab"
            aria-selected="false"
          >
            Installed
          </button>
        </li>
        <li class="nav-item" role="presentation">
          <button
            id="enabled-mods-tab"
            class="nav-link text-white"
            href="#"
            @click="showEnabled"
            role="tab"
            aria-selected="false"
          >
            Enabled
          </button>
        </li>
        <li class="dropdown">
          <a
            id="profiles-dropdown"
            class="nav-link dropdown-toggle text-white"
            data-bs-toggle="dropdown"
            href="#"
            role="button"
            aria-expanded="false"
            data-bs-auto-close="false"
            @click="checkCurrentProfile"
          >
            Profiles
          </a>
          <ul class="dropdown-menu p-1">
            <ModProfile
              v-for="(profile, index) in profiles"
              :profileName="profile.Name"
              :profileMods="profile.Mods"
              :key="index"
            />
            <button
              id="create-new-profile-button"
              class="btn btn-sm btn-primary m-1"
              data-bs-toggle="modal"
              data-bs-target="#create-profile-modal"
            >
              Create New Profile
            </button>
            <div class="btn-group d-flex justify-content-center mt-1">
              <button
                id="begin-export-profiles-button"
                class="btn btn-sm btn-success"
                @click="beginExportProfiles"
              >
                Export Profiles
              </button>
              <button
                id="import-profiles-button"
                class="btn btn-sm btn-primary"
                @click="importProfiles"
              >
                Import Profiles
              </button>
            </div>
            <div class="btn-group d-flex justify-content-center my-1">
              <button
                id="confirm-export-profiles-button"
                class="btn btn-small btn-success d-none"
                @click="exportProfiles"
              >
                Confirm
              </button>
              <button
                id="cancel-export-profiles-button"
                class="btn btn-sm btn-danger d-none"
                @click="cancelExportProfiles"
              >
                Cancel
              </button>
            </div>
          </ul>
        </li>
      </ul>
    </div>
    <button id="toggle-api-button" class="btn btn-danger" @click="toggleApi">
      Disable API
    </button>
    <button
      id="open-mods-button"
      class="btn btn-secondary btn-sm"
      @click="openMods"
    >
      Open Mods
    </button>
    <div class="input-group input-group-sm">
      <input
        type="search"
        id="mods-search"
        class="form-control input-sm"
        placeholder="Search mods"
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
        class="btn btn-success d-none"
        @click="createProfile"
      >
        Create Profile
      </button>
      <button
        id="cancel-create-profile-button"
        class="btn btn-danger d-none"
        @click="clearModProfileInputs"
      >
        Cancel
      </button>
    </div>
  </nav>
  <div
    id="create-profile-modal"
    class="modal fade"
    tabindex="-1"
    role="dialog"
    aria-hidden="true"
    aria-labelledby="create-new-profile-title"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h5 id="create-new-profile-title" class="modal-title">
            Give your profile a name
          </h5>
        </div>
        <div class="modal-body">
          <div class="input-group input-group-sm">
            <input
              type="text"
              id="profile-name-input"
              class="form-control input-sm"
              placeholder="Enter profile name here"
            />
          </div>
        </div>
        <div class="modal-footer">
          <button
            type="button"
            id="select-mods-button"
            class="btn btn-primary"
            data-bs-toggle="modal"
            data-bs-target="#create-profile-modal"
            @click="selectMods"
          >
            Select Mods
          </button>
          <button
            type="button"
            id="close-modal-button"
            class="btn btn-danger"
            data-bs-dismiss="modal"
            @click="clearModProfileInputs"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  </div>
  <div
    id="mod-details-container"
    style="padding: 65px 0px 0px 0px"
    data-bs-spy="scroll"
    data-bs-target="#nav-header"
    data-bs-offset="0"
  >
    <ModDetails
      v-for="(data, index) in modData"
      :mod="createModItem(data.Manifest.Name)"
      :modDescription="data.Manifest.Description"
      :modVersion="data.Manifest.Version"
      :modLink="data.Manifest.Link.$value"
      :sha256="data.Manifest.Link.SHA256"
      :dependencies="data.Manifest.Dependencies.Dependency"
      :key="index"
    />
  </div>
</template>

<script lang='ts'>
import "bootstrap";
import { defineComponent } from "vue";
import ModDetails from "./components/ModDetails.vue";
import { ModItem } from "./components/ModDetails.vue";
import ModProfile from "./components/ModProfile.vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "App",
  components: {
    ModDetails,
    ModProfile,
  },
  async mounted() {
    await this.reset();
    document.getElementById("all-mods-tab")?.click();
  },
  data() {
    return {
      enabledMods: [] as string[],
      installedMods: [] as string[],
      manifests: [] as any[],
      modData: [] as any[],
      modLinks: {},
      profiles: [] as any[],
      currentProfile: "",
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
      this.modData = [];
      await invoke("fetch_mod_list")
        .then((listString) => {
          this.modLinks = JSON.parse(listString as string);
          this.manifests = (this.modLinks as any).Manifest;
          this.manifests.forEach((manifest) =>
            this.modData.push({
              Manifest: manifest,
              Installed: false,
              Enabled: false,
            })
          );
        })
        .catch((e) => invoke("debug", { msg: e }));
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
     * Check whether the Modding API has been installed.
     */
    checkApiInstalled: async function (): Promise<void> {
      await invoke("check_api_installed")
        .then((installed) => {
          const toggleApiButton = document.getElementById(
            "toggle-api-button"
          ) as HTMLButtonElement;
          if (installed as boolean) {
            toggleApiButton.classList.remove("btn-success");
            toggleApiButton.classList.add("btn-danger");
            toggleApiButton.textContent = "Disable API";
          } else {
            toggleApiButton.classList.remove("btn-danger");
            toggleApiButton.classList.add("btn-success");
            toggleApiButton.textContent = "Enable API";
          }
        })
        .catch((e) => invoke("debug", { msg: e }));
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
        .catch((e) => invoke("debug", { msg: e }));
    },

    /**
     * Check the versions of all installed mods.
     */
    checkModVersions: async function (): Promise<void> {
      this.modData.forEach((data) => {
        const modName = data.Manifest.Name;
        const modVersionElement = document.getElementById(
          "mod-version-" + this.fitTextToAttribute(modName)
        ) as HTMLParagraphElement;
        const enableDisableButton = document.getElementById(
          "enable-disable-button-" + this.fitTextToAttribute(modName)
        ) as HTMLButtonElement;
        if (!enableDisableButton.classList.contains("d-none")) {
          invoke("check_for_update", {
            modName: modName,
            currentModVersion: modVersionElement.innerHTML.replace(
              " Value: ",
              ""
            ),
          }).then((outOfDate) => {
            const updateButton = document.getElementById(
              "update-button-" + this.fitTextToAttribute(modName)
            ) as HTMLButtonElement;
            if (outOfDate as boolean) {
              updateButton.classList.remove("d-none");
            } else {
              updateButton.classList.add("d-none");
            }
          });
        }
      });
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
      modDetailsContainer.setAttribute("style", "padding:65px 0px 0px 0px");
    },

    /**
     * Create a new ModItem instance
     * @return {ModItem} The newly created ModItem instance
     */
    createModItem: function (modName: string): ModItem {
      var installed = false;
      var enabled = false;
      if (this.installedMods.includes(modName)) {
        installed = true;
      }

      if (this.enabledMods.includes(modName)) {
        enabled = true;
      }

      var modItem = new ModItem(modName, installed, enabled);

      return modItem;
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
      const modDetailsContainer = document.getElementById(
        "mod-details-container"
      ) as HTMLDivElement;
      modDetailsContainer.setAttribute("style", "padding:65px 0px 0px 0px");
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

      await invoke("export_profiles", { profileNames: profileNames });
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
     * Get all mods that are installed and all mods that are enabled and modify mod data accordingly.
     */
    getInstalledAndEnabledMods: async function (): Promise<void> {
      await invoke("fetch_enabled_mods")
        .then((enabledMods: any) => {
          (enabledMods as Array<any>).forEach((enabled) => {
            this.enabledMods.push(enabled.Name);
          });
        })
        .catch((e) => invoke("debug", { msg: e }));
      await invoke("fetch_installed_mods")
        .then((installedMods: any) => {
          (installedMods as Array<any>).forEach((installed) => {
            this.installedMods.push(installed.Name);
          });
        })
        .catch((e) => invoke("debug", { msg: e }));
    },

    /**
     * Get all manually installed mods and add them to the mod list.
     */
    getManuallyInstalledMods: async function (): Promise<void> {
      await invoke("fetch_manually_installed_mods")
        .then((json) => {
          const manuallyInstalledMods = JSON.parse(json as string);
          manuallyInstalledMods.forEach((mod: { name: any; enabled: any }) => {
            const manifest = {
              Name: mod.name,
              Description: "No description available.",
              Version: "Unknown",
              Link: "",
              Dependencies: [],
            };

            this.modData.push({
              Manifest: manifest,
              Installed: true,
              Enabled: mod.enabled,
            });
          });
        })
        .catch((e) => invoke("debug", { msg: e }));
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
        .catch((e) => invoke("debug", { msg: e }));
    },

    importProfiles: async function (): Promise<void> {
      await invoke("import_profiles");
      this.reset();
    },

    /**
     * Open the local folder on the file system containing all installed mods.
     */
    openMods: function (): void {
      invoke("open_mods_folder");
    },

    /**
     * Build all mod data again.
     */
    reset: async function (): Promise<void> {
      await this.checkApiInstalled();
      await this.buildModList();
      await this.getInstalledAndEnabledMods();
      await this.getManuallyInstalledMods();
      await this.getProfiles();
      await this.checkModVersions();

      this.modData.sort((a: any, b: any) =>
        a.Manifest.Name > b.Manifest.Name ? 1 : -1
      );
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
        const enable_disable_button = details.querySelector(
          ".enable-disable-button"
        ) as HTMLButtonElement;
        const install_uninstall_button = details.querySelector(
          ".install-uninstall-button"
        ) as HTMLButtonElement;
        if (
          (modName.includes(value) || modDesc.includes(value)) &&
          (document
            .getElementById("all-mods-tab")
            ?.classList.contains("active") ||
            (document
              .getElementById("enabled-mods-tab")
              ?.classList.contains("active") &&
              !enable_disable_button.classList.contains("d-none") &&
              enable_disable_button.textContent == "Disable") ||
            (document
              .getElementById("installed-mods-tab")
              ?.classList.contains("active") &&
              install_uninstall_button.textContent == "Uninstall"))
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
      const modDetailsContainer = document.getElementById(
        "mod-details-container"
      ) as HTMLDivElement;
      modDetailsContainer.setAttribute("style", "padding:110px 0px 0px 0px");
      document
        .getElementById("create-profile-button")
        ?.classList.remove("d-none");
      document
        .getElementById("cancel-create-profile-button")
        ?.classList.remove("d-none");
    },

    /**
     * Activate the "All" tab.
     */
    showAll: function (): void {
      const tabs = document.querySelectorAll("#nav-header ul li button");
      tabs.forEach((tab) => {
        if (tab.id == "all-mods-tab" && !tab.classList.contains("active")) {
          tab.classList.add("active");
          tab.classList.remove("text-white");
        } else if (tab.classList.contains("active")) {
          tab.classList.remove("active");
          tab.classList.add("text-white");
        }
      });
      this.searchMods();
    },

    /**
     * Activate the "Enabled" tab.
     */
    showEnabled: function (): void {
      const tabs = document.querySelectorAll("#nav-header ul li button");
      tabs.forEach((tab) => {
        if (tab.id == "enabled-mods-tab" && !tab.classList.contains("active")) {
          tab.classList.add("active");
          tab.classList.remove("text-white");
        } else if (tab.classList.contains("active")) {
          tab.classList.remove("active");
          tab.classList.add("text-white");
        }
      });

      this.searchMods();
    },

    /**
     * Activate the "Installed" tab.
     */
    showInstalled: function (): void {
      const tabs = document.querySelectorAll("#nav-header ul li button");
      tabs.forEach((tab) => {
        if (
          tab.id == "installed-mods-tab" &&
          !tab.classList.contains("active")
        ) {
          tab.classList.add("active");
          tab.classList.remove("text-white");
        } else if (tab.classList.contains("active")) {
          tab.classList.remove("active");
          tab.classList.add("text-white");
        }
      });

      this.searchMods();
    },

    /**
     * Toggle the Modding API.
     */
    toggleApi: async function (): Promise<void> {
      await invoke("toggle_api")
        .then((enabled) => {
          const toggleApiButton = document.getElementById(
            "toggle-api-button"
          ) as HTMLButtonElement;
          if (enabled) {
            toggleApiButton.classList.remove("btn-success");
            toggleApiButton.classList.add("btn-danger");
            toggleApiButton.textContent = "Disable API";
          } else {
            toggleApiButton.classList.remove("btn-danger");
            toggleApiButton.classList.add("btn-success");
            toggleApiButton.textContent = "Enable API";
          }
        })
        .catch((e) => invoke("debug", { msg: e }));
    },
  },
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: left;
  color: black;
  margin-top: 60px;
}

.accordion-button::after {
  content: none;
}

@import "~bootstrap/dist/css/bootstrap.css";
</style>
