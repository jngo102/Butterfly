<template>
  <div id="mod-links">
    <ModDetails v-for='(manifest, index) in modLinks.Manifest'
                :mod='createModItem(manifest.Name, this.installed[index], this.enabled[index])'
                :modDescription='manifest.Description'
                :modVersion='manifest.Version'
                :modLink='manifest.Link'
                :sha256='manifest.SHA256'
                :dependencies='manifest.Dependencies.Dependency'
                :key='index' />
  </div>
</template>

<script lang="ts">
import 'bootstrap'
import { defineComponent } from 'vue';
import ModDetails from './components/ModDetails.vue';
import { ModItem } from './components/ModDetails.vue';
import { invoke } from '@tauri-apps/api/tauri';

export default defineComponent({
  name: 'App',
  components: {
    ModDetails
  },
  async mounted() {
    await this.buildModList();
    await this.getInstalledAndEnabledMods();
  },
  data() {
    return {
      modLinks: {},
      installed: [false],
      enabled: [false],
    }
  },
  methods: {
    buildModList: async function(): Promise<void> {
      await invoke('fetch_mod_list')
        .then((listString) => {
          this.modLinks = JSON.parse(listString as string);
          invoke('debug', { msg: JSON.stringify(this.modLinks, null, 4) });
        })
        .catch((e) => {
          console.error(e);
          invoke('debug', {msg: e});
        });
    },

    createModItem: function(modName: string, installed: boolean, enabled: boolean): ModItem {
      return new ModItem(modName, installed, enabled);
    },

    getInstalledAndEnabledMods: async function(): Promise<void> {
      await invoke('fetch_enabled_mods')
        .then((enabled: any) => {
          this.enabled = (enabled as Array<boolean>);
        })
        .catch((e) => console.error(e));
      await invoke('fetch_installed_mods')
        .then((installed: any) => this.installed = (installed as Array<boolean>))
        .catch((e) => console.error(e));
    }
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
