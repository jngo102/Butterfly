<template>
  <div id="mod-links">
    <ModDetails v-for="(manifest, index) in modLinks.Manifest"
                :modName="manifest.Name"
                :modDescription="manifest.Description"
                :modVersion="manifest.Version"
                :modLink="manifest.Link"
                :sha256="manifest.SHA256"
                :dependencies="manifest.Dependencies.Dependency"
                :key="index" />
  </div>
</template>

<script lang="ts">
import 'bootstrap'
import { defineComponent } from 'vue';
import ModDetails from './components/ModDetails.vue';
import { invoke } from '@tauri-apps/api/tauri';

export default defineComponent({
  name: 'App',
  components: {
    ModDetails
  },
  mounted() {
    this.buildModList();
  },
  data() {
    return {
      modLinks: {},
    }
  },
  methods: {
    buildModList: function() {
      invoke('fetch_mod_list')
        .then((listString) => {
          this.modLinks = JSON.parse(listString as string);
          invoke('debug', { msg: JSON.stringify(this.modLinks, null, 4) });
        })
        .catch((e) => {
          console.error(e);
          invoke('debug', {msg: e});
        });
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
@import '~bootstrap/dist/css/bootstrap.css';
</style>
