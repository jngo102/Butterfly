<template>
  <div class='accordion accordion-flush mod-details' :id='"mod-details-"+fitTextToAttribute(mod.name)'>
    <div class='accordion-item' :id='"mod-main-"+fitTextToAttribute(mod.name)'>
      <div class='accordion-header' :id='"mod-header-"+fitTextToAttribute(mod.name)'>
        <button class='accordion-button collapsed row'
                data-bs-toggle='collapse'
                :data-bs-target='"#collapsed-details-"+fitTextToAttribute(mod.name)' 
                aria-expanded='false"'
                :aria-controls='"collapsed-details-"+fitTextToAttribute(mod.name)'>
          <p class='col align-self-center mod-name'>{{ mod.name }}</p>
          <input :id='"mod-link-"+fitTextToAttribute(mod.name)' class='mod-link' type='hidden' :value='modLink' />
          <p class='col align-self-center'>Version: {{ modVersion }}</p>
          <button class='btn btn-success col align-self-center install-uninstall-button'
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
        <div class='accordion-body'>
          <p class='mod-description'>{{ modDescription }}</p>
          <div class='dependencies'>
            <h3>Dependencies</h3>
            <ul :id='"dependency-"+fitTextToAttribute(mod.name)'>
              <li v-for='dependency in dependencies' :key='dependency'>
                  {{ dependency }}
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang='ts'>
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
    /**
     * Either enables or disables a mod depending on the mod's current enabled status.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    enableOrDisableMod: function(event: MouseEvent): void {
      let enableDisableButton = document.getElementById('enable-disable-button'+
        this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (event.target != enableDisableButton) return;
      invoke(this.mod?.enabled ? 'disable_mod' : 'enable_mod', 
        { modName: this.mod?.name });
      (this.mod as ModItem).enabled = !this.mod?.enabled;
      enableDisableButton.textContent = this.mod?.enabled ? "Disable" : "Enable";
    },

    /**
     * Modifies text so that it may be used in an attribute, i.e. removing spaces
     * and non-alphanumeric characters.
     * @param {string} text The text to be modified
     * @return {string}     The modified text
     */
    fitTextToAttribute: function(text: string): string {
      return text.replace(/\W+/g, "");
    },

    /**
     * Get the class name of the enable/disable mod button.
     * @return {string} The class to be applied to the enable/disable button
     */
    getButtonClass: function(): string {
      let classAttribute = 'btn btn-success col align-self-center enable-disable-button';
      classAttribute += this.mod?.installed ? '' : ' d-none';
      return classAttribute;
    },

    /**
     * Install a mod.
     * @param {string} modName The name of the mod to be installed
     * @param {string} modLink The link to the download of the mod to be installed
     */
    installMod: function(modName: string, modLink: string): void {
      invoke('install_mod', { modName: modName, modLink: modLink });
      var dependencyElement = document.getElementById('dependency-' + this.fitTextToAttribute(modName)) as HTMLUListElement;
      let dependencies = dependencyElement.querySelectorAll('li');
      dependencies.forEach((dep) => {
        invoke('debug', { msg: "Installing dependency: " + dep.innerText });
        var modLinkElement = document.getElementById('mod-link-' + this.fitTextToAttribute(dep.innerText)) as HTMLInputElement;
        this.installMod(dep.innerText, modLinkElement.value);
        let installUninstallButton = document.getElementById('install-uninstall-button'+
          this.fitTextToAttribute(dep.innerText)) as HTMLButtonElement;
        let enableDisableButton = document.getElementById('enable-disable-button'+
          this.fitTextToAttribute(dep.innerText)) as HTMLButtonElement;
        enableDisableButton.classList.remove('d-none');
        enableDisableButton.textContent = "Disable";
        installUninstallButton.textContent = "Uninstall"; 
      });
    },

    /**
     * Either installs or uninstalls a mod depending on its current installation status.
     * Also automatically installs the mod's dependencies.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    installOrUninstallMod: function(event: MouseEvent): void {
      let installUninstallButton = document.getElementById('install-uninstall-button'+
          this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (event.target != installUninstallButton) return;
      let enableDisableButton = document.getElementById('enable-disable-button'+
          this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (this.mod?.installed) {
        invoke('uninstall_mod', { modName: this.mod?.name });
        enableDisableButton.classList.add('d-none');
        installUninstallButton.textContent = "Install";
      } else {
        this.installMod(this.mod?.name as string, this.modLink as string);
        enableDisableButton.classList.remove('d-none');
        enableDisableButton.textContent = "Disable";
        installUninstallButton.textContent = "Uninstall";
      }
      
      (this.mod as ModItem).installed = !this.mod?.installed;
      (this.mod as ModItem).enabled = true;
    },
  }
});
</script>

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
