<template>
  <nav id='nav-header' class='nav navbar navbar-dark fixed-top bg-dark'>
    <ul id='filter-tabs' class='nav nav-tabs bg-dark' role='tablist'>
      <li class='nav-item' role='presentation'>
        <button id='all-mods-tab'
                class='nav-link active' 
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
                class='nav-link' 
                href='#' 
                @click='showInstalled()' 
                role='tab'
                aria-selected='false'>
          Installed
        </button>
      </li>
      <li class='nav-item' role='presentation'>
        <button id='enabled-mods-tab' 
                class='nav-link' 
                href='#' 
                @click='showEnabled()' 
                role='tab'
                aria-selected='false'>
          Enabled
        </button>
      </li>
    </ul>
    <div class='input-group'>
      <div class='form-horizontal'>
        <input type='search' id='mods-search' class='form-control' placeholder="Search mods" @input='searchMods()' />
      </div>
    </div>
  </nav>
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
import { invoke } from '@tauri-apps/api/tauri'

export default defineComponent({
  name: 'App',
  components: {
    ModDetails
  },
  async mounted() {
    await this.reset();
  },
  data() {
    return {
      manifests: [] as any[],
      modData: [] as any[],
      modLinks: {},
      installed: [] as boolean[],
      enabled: [] as boolean[],
    }
  },
  methods: {
    buildModList: async function(): Promise<void> {
      this.modData = [];
      await invoke('fetch_mod_list')
        .then((listString) => {
          this.modLinks = JSON.parse(listString as string);
          this.manifests = (this.modLinks as any).Manifest;
          this.manifests.forEach(manifest => this.modData.push({"Manifest": manifest, "Installed": false, "Enabled": false}));
        })
        .catch((e) => {
          console.error(e);
          invoke('debug', { msg: e });
        });
    },

    createModItem: function(modName: string, installed: boolean, enabled: boolean): ModItem {
      return new ModItem(modName, installed, enabled);
    },

    getInstalledAndEnabledMods: async function(): Promise<void> {
      await invoke('fetch_enabled_mods')
        .then((enabled: any) => {
          this.enabled = enabled as Array<boolean>;
          this.enabled.forEach((enabled, index) => this.modData[index].Enabled = enabled);
        })
        .catch((e) => console.error(e));
      await invoke('fetch_installed_mods')
        .then((installed: any) => {
          this.installed = installed as Array<boolean>;
          this.installed.forEach((installed, index) => this.modData[index].Installed = installed);
        })
        .catch((e) => console.error(e));
      this.modData.sort((a: any, b: any) => a.Manifest.Name > b.Manifest.Name ? 1 : -1);
    },

    reset: async function(): Promise<void> {
      await this.buildModList();
      await this.getInstalledAndEnabledMods();
    },

    searchMods: async function(): Promise<void> {
      await this.reset();
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

    showAll: function(): void {
      const tabs = document.querySelectorAll('#nav-header ul li button');
      tabs.forEach((tab) => {
        if (tab.id == 'all-mods-tab' && !tab.classList.contains('active')) {
          tab.classList.add('active');
        } else if (tab.classList.contains('active')) {
          tab.classList.remove('active');
        }
      });
      const modDetails = document.querySelectorAll('.mod-details');
      modDetails.forEach((details) => {
        if (details.classList.contains('d-none')) {
          details.classList.remove('d-none');
        }
      });
    },

    showEnabled: function(): void {
      const tabs = document.querySelectorAll('#nav-header ul li button');
      tabs.forEach((tab) => {
        if (tab.id == 'enabled-mods-tab' && !tab.classList.contains('active')) {
          tab.classList.add('active');
        } else if (tab.classList.contains('active')) {
          tab.classList.remove('active');
        }
      });
      const modDetails = document.querySelectorAll('.mod-details');
      modDetails.forEach((details) => {
        const enable_disable_button = details.querySelector('.enable-disable-button') as HTMLButtonElement;
        if (enable_disable_button?.textContent == "Enable") {
          details.classList.add('d-none');
        } else if (!enable_disable_button?.classList.contains('d-none') &&
                   details.classList.contains('d-none')) {
          details.classList.remove('d-none');
        }
      });
    },

    showInstalled: function(): void {
      const tabs = document.querySelectorAll('#nav-header ul li button');
      tabs.forEach((tab) => {
        if (tab.id == 'installed-mods-tab' && !tab.classList.contains('active')) {
          tab.classList.add('active');
        } else if (tab.classList.contains('active')) {
          tab.classList.remove('active');
        }
      });
      const modDetails = document.querySelectorAll('.mod-details');
      modDetails.forEach((details) => {
        const install_uninstall_button = details.querySelector('.install-uninstall-button') as HTMLButtonElement;
        if (install_uninstall_button?.textContent == "Install") {
          details.classList.add('d-none');
        } else if (details.classList.contains('d-none')) {
          details.classList.remove('d-none');
        }
      });
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
