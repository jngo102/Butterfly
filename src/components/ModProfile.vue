<template>
  <li :id='fitTextToAttribute(profileName)+"-check"' class='dropdown-item form-check'>
    <input :id='fitTextToAttribute(profileName)+"-radio"'
           class='form-check-input'
           type='radio'
           name='mod-profiles'
           @change='changeProfile()'/>
    <label class='form-check-label' :for='fitTextToAttribute(profileName)+"-radio"'>
      {{ profileName }}
    </label>
    <input v-for='(mod, index) in profileMods'
          :id='profileName+"-"+mod'
          class='profile-mod-name'
          type='hidden'
          :value='mod'
          :key='index' />
  </li>
</template>

<script lang='ts'>
import { defineComponent } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';

export default defineComponent({
  name: 'ModProfile',
  props: {
    profileName: String,
    profileMods: Array,
  },
  methods: {
    changeProfile: function(): void {
      invoke('set_profile', { profileName: this.profileName });
      const li = document.getElementById(this.fitTextToAttribute(this.profileName as string)+'-check');
      const radioButton = li?.querySelector('.form-check-input') as HTMLInputElement;
      const checked = radioButton.value;
      if (checked != 'on') return;
      const modNameInputs = li?.querySelectorAll('.profile-mod-name');
      var modNames: Array<string> = [];
      modNameInputs?.forEach((input) => {
        const inputValue = (input as HTMLInputElement).value;
        modNames.push(inputValue);
        var modLinkElement = document.getElementById('mod-link-' + this.fitTextToAttribute(inputValue)) as HTMLInputElement;
        this.installMod(inputValue, modLinkElement.value);
      });
      const allModRows = document.querySelectorAll('.mod-details-row');
      allModRows.forEach((row) => {
        const modName = row.querySelector('.mod-name')?.textContent as string;
        if (!modNames.includes(modName)) {
          invoke('disable_mod', { modName: modName });
        }
      });
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
  }
});
</script>