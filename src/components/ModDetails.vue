<template>
  <div class='accordion accordion-flush' :id='"mod-details-"+fitTextToAttribute(mod.name)'>
    <div class='accordion-item' :id='"mod-main-"+fitTextToAttribute(mod.name)'>
      <div class='accordion-header' :id='"mod-header-"+fitTextToAttribute(mod.name)'>
        <button class='accordion-button collapsed row'
                data-bs-toggle='collapse'
                :data-bs-target='"#collapsed-details-"+fitTextToAttribute(mod.name)' 
                aria-expanded='false"'
                :aria-controls='"collapsed-details-"+fitTextToAttribute(mod.name)'>
          <p class='col align-self-center'>{{ mod.name }}</p>
          <p class='col align-self-center'>Version: {{ modVersion }}</p>
          <button class='btn btn-success col align-self-center'
                  :id='"install-uninstall-button"+fitTextToAttribute(mod.name)'
                  @click='installOrUninstallMod'>
            {{ mod.installed ? "Uninstall" : "Install" }}
          </button>
          <button :class='getButtonClass()'
                  :id='"enable-disable-button"+fitTextToAttribute(mod.name)'
                  @click='enableOrDisableMod'>
            {{ mod.enabled ? "Disable" : "Enable" }}
          </button>
        </button>
      </div>
      <div class='accordion-collapse collapse'
           :id='"collapsed-details-"+fitTextToAttribute(mod.name)' 
           :aria-labelledby='"mod-header-"+fitTextToAttribute(mod.name)' 
           :data-bs-parent='"#mod-details-"+fitTextToAttribute(mod.name)'>
        <div class='mod-description accordion-body'>
          <p>{{ modDescription }}</p>
          <div class='dependencies'>
            <h3>Dependencies</h3>
            <li>
              <ul v-for='dependency in dependencies' :key='dependency'>
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
  name: 'ModDetails',
  props: {
    mod: ModItem,
    modDescription: String,
    modVersion: String,
    modLink: String,
    sha256: String,
    dependencies: Array,
  },
  methods: {
    enableOrDisableMod(event: MouseEvent) {
      let enableDisableButton = document.getElementById('enable-disable-button'+
        this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (event.target != enableDisableButton) return;
      invoke(this.mod?.enabled ? 'disable_mod' : 'enable_mod', 
        { modName: this.mod?.name });
      (this.mod as ModItem).enabled = !this.mod?.enabled;
      enableDisableButton.textContent = this.mod?.enabled ? "Disable" : "Enable";
    },

    fitTextToAttribute(attribute: string): string {
      return attribute.replace(/\W+/g, "");
    },

    getButtonClass(): string {
      let classAttribute = 'btn btn-success col align-self-center';
      classAttribute += this.mod?.installed ? '' : ' d-none';
      return classAttribute;
    },

    installOrUninstallMod(event: MouseEvent) {
      let installUninstallButton = document.getElementById('install-uninstall-button'+
          this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (event.target != installUninstallButton) return;
      let enableDisableButton = document.getElementById('enable-disable-button'+
          this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (this.mod?.installed) {
        invoke('uninstall_mod', { modName: this.mod?.name });
        enableDisableButton.className += ' d-none';
        installUninstallButton.textContent = "Install";
      } else {
        invoke('install_mod', { modName: this.mod?.name, modLink: this.modLink });
        enableDisableButton.className = enableDisableButton.className.replace(' d-none', '');
        enableDisableButton.textContent = "Disable";
        installUninstallButton.textContent = "Uninstall";
      }
      
      (this.mod as ModItem).installed = !this.mod?.installed;
      (this.mod as ModItem).enabled = true;
    },
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
