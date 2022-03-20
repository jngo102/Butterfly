<template>
  <nav id='nav-header' class='nav navbar navbar-dark nav-justified justify-content-center fixed-top bg-dark'>
      <ul id='filter-tabs' class='nav nav-tabs nav-justified bg-dark justify-content-center' role='tablist'>
          <li class='nav-item' role='presentation'>
              <button id='all-mods-tab'
                              class='nav-link text-white active' 
                              aria-current='page' 
                              href='#' 
                              @click='showAll()' 
                              role='tab'
                              aria-selected='true'>
                  All
              </button> 
          </li>
          <li class='nav-item' role='presentation'> 
              <button id='installed-mods-tab' 
                              class='nav-link text-white'
                              href='#' 
                              @click='showInstalled()' 
                              role='tab'
                              aria-selected='false'>
                  Installed
              </button>
          </li>
          <li class='nav-item' role='presentation'>
              <button id='enabled-mods-tab' 
                      class='nav-link text-white' 
                      href='#' 
                      @click='showEnabled()' 
                      role='tab'
                      aria-selected='false'>
                  Enabled
              </button>
          </li>
          <li>
            <a class='nav-link dropdown-toggle' data-bs-toggle='dropdown' href='#' role='button' aria-expanded='false'>
                Profiles
            </a>
            <ul class='dropdown-menu'>
                <ModProfile v-for='(profile, index) in profiles'
                            :profileName='profile.Name'
                            :profileMods='profile.Mods'
                            :key='index' />
                <button id='create-new-profile-button'
                        class='btn btn-primary'
                        data-bs-toggle='modal'
                        data-bs-target='#create-profile-modal'>
                  Create New Profile
                </button>
            </ul>
          </li>
      </ul>
      <div class='input-group input-group-sm'>
          <input type='search' 
                 id='mods-search' 
                 class='form-control input-sm' 
                 placeholder="Search mods" 
                 @input='searchMods()' />
      </div>
  </nav>
  <div id='create-profile-modal' 
       class='modal fade' 
       tabindex='-1' 
       role='dialog' 
       aria-hidden='true' 
       aria-labelledby='create-new-profile-title'>
    <div class='modal-dialog' role='document'>
      <div class='modal-content'>
        <div class='modal-header'>
          <h5 id='create-new-profile-title' class='modal-title'>
            Create a new profile
          </h5>
        </div>
        <div class='modal-body'>
          <div class='input-group input-group-sm'>
            <input type='text' id='profile-name-input' class='form-control input-sm' placeholder="Enter profile name here"/>
          </div>
          <ul class='profile-mods' data-bs-spy='scroll' tab-index='0'>
              <ProfileMod v-for='(data, index) in modData'
                          :modName='data.Manifest.Name'
                          :key='index' />
          </ul>
        </div>
        <div class="modal-footer">
          <button type="button" id='create-profile-button' class="btn btn-primary" @click='createProfile()'>Create Profile</button>
          <button type="button" id='close-modal-button' class="btn btn-danger" data-bs-dismiss="modal">Cancel</button>
        </div>
      </div>
    </div>
  </div>
  <div id='mod-details' style='padding:50px 0px 0px 0px' data-bs-spy='scroll' data-bs-target='#nav-header' data-bs-offset='0'>
    <ModDetails v-for='(data, index) in modData'
                :mod='createModItem(data.Manifest.Name, data.Installed, data.Enabled)'
                :modDescription='data.Manifest.Description'
                :modVersion='data.Manifest.Version'
                :modLink='data.Manifest.Link'
                :sha256='data.Manifest.SHA256'
                :dependencies='data.Manifest.Dependencies.Dependency'
                :key='index' />
  </div>
</template>

<script lang='ts'>
import 'bootstrap'
import { defineComponent } from 'vue'
import ModDetails from './components/ModDetails.vue'
import { ModItem } from './components/ModDetails.vue'
import ModProfile from './components/ModProfile.vue'
import ProfileMod from './components/ProfileMod.vue'
import { invoke } from '@tauri-apps/api/tauri'

export default defineComponent({
    name: 'App',
    components: {
        ModDetails,
        ModProfile,
        ProfileMod,
    },
    mounted() {
        this.reset();
    },
    data() {
        return {
            enabled: [] as boolean[],
            installed: [] as boolean[],
            manifests: [] as any[],
            modData: [] as any[],
            modLinks: {},
            profiles: [] as any[],
        }
    },
    methods: {
        /**
         * Fetch the mod links JSON from backend to build mod data.
         */
        buildModList: function(): void {
            this.modData = [];
            invoke('fetch_mod_list')
                .then((listString) => {
                    this.modLinks = JSON.parse(listString as string);
                    this.manifests = (this.modLinks as any).Manifest;
                    this.manifests.forEach(manifest => this.modData.push({"Manifest": manifest, "Installed": false, "Enabled": false}));
                })
                .catch((e) => invoke('debug', { msg: e }));
        },

        /**
         * Create a new ModItem instance
         * @return {ModItem} The newly created ModItem instance
         */
        createModItem: function(modName: string, installed: boolean, enabled: boolean): ModItem {
            return new ModItem(modName, installed, enabled);
        },

        /**
         * Create a new mod profile
         */
        createProfile: function() {
          const profileNameInput = document.getElementById('profile-name-input') as HTMLInputElement;
          const profileMods = document.querySelectorAll('.profile-mod');
          var modNames: Array<string> = [];
          profileMods.forEach((mod) => {
            if ((mod.querySelector('.profile-mod-checkbox') as HTMLInputElement).checked) {
              const modName = mod.querySelector('.profile-mod-label')?.textContent as string;
              modNames.push(modName);
            }
          });
          invoke('create_profile', { profileName: profileNameInput.value, modNames: modNames });
          profileNameInput.value = "";
          profileMods.forEach((mod) => {
            (mod.querySelector('input') as HTMLInputElement).checked = false;
          });
          (document.getElementById('close-modal-button') as HTMLButtonElement).click();
        },

        /**
         * Get all mods that are installed and all mods that are enabled and modify mod data accordingly.
         */
        getInstalledAndEnabledMods: function(): void {
            invoke('fetch_enabled_mods')
                .then((enabled: any) => {
                    this.enabled = enabled as Array<boolean>;
                    this.enabled.forEach((enabled, index) => this.modData[index].Enabled = enabled);
                })
                .catch((e) => invoke('debug', { msg: e }));
            invoke('fetch_installed_mods')
                .then((installed: any) => {
                    this.installed = installed as Array<boolean>;
                    this.installed.forEach((installed, index) => this.modData[index].Installed = installed);
                })
                .catch((e) => invoke('debug', { msg: e }));
            this.modData.sort((a: any, b: any) => a.Manifest.Name > b.Manifest.Name ? 1 : -1);
        },

        /**
         * Get all mod profiles from app settings.
         */
        getProfiles: function(): void {
            invoke('fetch_profiles')
                .then((profilesString: any) => {
                    invoke('debug', { msg: "Profiles string: " + (profilesString as string) });
                    if ((profilesString as string) != "[]") {
                      const profiles = JSON.parse(profilesString as string);
                      this.profiles = profiles as Array<any>;
                      invoke('debug', { msg: "Assigned profiles: " + this.profiles });
                    }
                })
                .catch((e) => invoke('debug', { msg: e }));
        },

        /**
         * Build all mod data again.
         */
        reset: function(): void {
            this.buildModList();
            this.getInstalledAndEnabledMods();
            this.getProfiles();
        },

        /**
         * Filter the mod list based on search input.
         */
        searchMods: function(): void {
            const value = (document.getElementById('mods-search') as HTMLInputElement).value?.toLowerCase() as string;
            invoke('debug', { msg: "Search input: " + value });
            const modDetails = document.querySelectorAll('.mod-details');
            modDetails.forEach((details) => {
                const modName = details.querySelector('.mod-name')?.textContent?.toLowerCase() as string;
                const modDesc = details.querySelector('.mod-description')?.textContent?.toLowerCase() as string;
                const enable_disable_button = details.querySelector('.enable-disable-button') as HTMLButtonElement;
                const install_uninstall_button = details.querySelector('.install-uninstall-button') as HTMLButtonElement;
                if ((modName.includes(value) || modDesc.includes(value)) &&
                        (document.getElementById('all-mods-tab')?.classList.contains('active') ||
                        (document.getElementById('enabled-mods-tab')?.classList.contains('active') && enable_disable_button?.textContent == "Disable") ||
                        (document.getElementById('installed-mods-tab')?.classList.contains('active') && install_uninstall_button?.textContent == "Uninstall"))) {
                    details.classList.remove('d-none');
                } else {
                    details.classList.add('d-none');
                }
            });
        },

        /**
         * Activate the "All" tab.
         */
        showAll: function(): void {
            const tabs = document.querySelectorAll('#nav-header ul li button');
            tabs.forEach((tab) => {
                if (tab.id == 'all-mods-tab' && !tab.classList.contains('active')) {
                    tab.classList.add('active');
                    tab.classList.remove('text-white');
                } else if (tab.classList.contains('active')) {
                    tab.classList.remove('active');
                    tab.classList.add('text-white');
                }
            });
            this.searchMods();
        },

        /**
         * Activate the "Enabled" tab.
         */
        showEnabled: function(): void {
            const tabs = document.querySelectorAll('#nav-header ul li button');
            tabs.forEach((tab) => {
                if (tab.id == 'enabled-mods-tab' && !tab.classList.contains('active')) {
                    tab.classList.add('active');
                    tab.classList.remove('text-white');
                } else if (tab.classList.contains('active')) {
                    tab.classList.remove('active');
                    tab.classList.add('text-white');
                }
            });
            this.searchMods();
        },

        /**
         * Activate the "Installed" tab.
         */
        showInstalled: function(): void {
            const tabs = document.querySelectorAll('#nav-header ul li button');
            tabs.forEach((tab) => {
                if (tab.id == 'installed-mods-tab' && !tab.classList.contains('active')) {
                    tab.classList.add('active');
                    tab.classList.remove('text-white');
                } else if (tab.classList.contains('active')) {
                    tab.classList.remove('active');
                    tab.classList.add('text-white');
                }
            });
            this.searchMods();
        },
    }
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

@import '~bootstrap/dist/css/bootstrap.css';
</style>
