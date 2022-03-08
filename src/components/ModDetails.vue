<template>
  <div class='accordion' :id='"mod-details-"+replaceSpacesForAttribute(modName)'>
    <div class='accordion-item' :id='"mod-main-"+replaceSpacesForAttribute(modName)'>
      <div class='accordion-header' :id='"mod-header-"+replaceSpacesForAttribute(modName)'>
        <button class='accordion-button collapsed row'
                type='button'
                data-bs-toggle='collapse'
                :data-bs-target='"#collapsed-details-"+replaceSpacesForAttribute(modName)' 
                aria-expanded='false"'
                :aria-controls='"collapsed-details-"+replaceSpacesForAttribute(modName)'>
          <p class="col align-self-center">{{ modName }}</p>
          <p class="col align-self-center">Version: {{ modVersion }}</p>
          <button class="download-button btn btn-success col align-self-center" @click="installMod()">
            Install
          </button>
        </button>
      </div>
      <div class="accordion-collapse collapse" 
           :id='"collapsed-details-"+replaceSpacesForAttribute(modName)' 
           :aria-labelledby='"mod-header-"+replaceSpacesForAttribute(modName)' 
           :data-bs-parent='"#mod-details-"+replaceSpacesForAttribute(modName)'>
        <div class="mod-description accordion-body">
          <p>{{ modDescription }}</p>
          <div class="dependencies">
            <h3>Dependencies</h3>
            <li>
              <ul v-for="dependency in dependencies"
                  :key="dependency">
                  {{ dependency }}
              </ul>
            </li>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import 'bootstrap';
import { defineComponent } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

export default defineComponent({
  name: 'ModDetails',
  props: {
    modName: String,
    modDescription: String,
    modVersion: String,
    modLink: String,
    sha256: String,
    dependencies: Array,
  },
  methods: {
    installMod() {
      invoke('install_mod', { modName: this.modName, modLink: this.modLink })
        .then((msg) => console.log(msg));
    },
    replaceSpacesForAttribute(attribute: string): string {
      return attribute.replaceAll(" ", "-");
    }
  }
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
