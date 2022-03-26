<template>
  <div class='accordion accordion-flush mod-details' :id='"mod-details-"+fitTextToAttribute(mod.name)'>
    <div class='accordion-item' :id='"mod-main-"+fitTextToAttribute(mod.name)'>
      <div class='accordion-header form-check' :id='"mod-header-"+fitTextToAttribute(mod.name)'>
        <button class='d-flex accordion-button collapsed row mod-details-row'
                data-bs-toggle='collapse'
                :data-bs-target='"#collapsed-details-"+fitTextToAttribute(mod.name)' 
                aria-expanded='false"'
                :aria-controls='"collapsed-details-"+fitTextToAttribute(mod.name)'>
          <input :id='"profile-checkbox-"+fitTextToAttribute(mod.name)'
              class='profile-mod-checkbox form-check-input d-none'
              type='checkbox'
              @click='doNotOpenAccordion(this.event)' />
          <p class='col align-self-center mod-name'>{{ mod.name }}</p>
          <input :id='"mod-link-"+fitTextToAttribute(mod.name)' class='mod-link' type='hidden' :value='modLink' />
          <input :id='"mod-hash-"+fitTextToAttribute(mod.name)' class='mod-hash' type='hidden' :value='sha256' />
          <p :id='"mod-version-"+fitTextToAttribute(mod.name)' class='col align-self-center mod-version'>
            Version: {{ modVersion }}
          </p>
          <button class='btn btn-success col align-self-center install-uninstall-button px-3'
                  :id='"install-uninstall-button"+fitTextToAttribute(mod.name)'
                  @click='installOrUninstallMod'>
            {{ mod.installed ? "Uninstall" : "Install" }}
          </button>
          <button :class='getButtonClass()'
                  :id='"enable-disable-button-"+fitTextToAttribute(mod.name)'
                  @click='enableOrDisableMod'>
            {{ mod.enabled ? "Disable" : "Enable" }}
          </button>
          <button :id='"update-button-"+fitTextToAttribute(mod.name)' 
                  class='btn btn-warning d-none'
                  @click='updateMod()'>
            Update
          </button>
        </button>
      </div>
      <div class='accordion-collapse collapse'
           :id='"collapsed-details-"+fitTextToAttribute(mod.name)' 
           :aria-labelledby='"mod-header-"+fitTextToAttribute(mod.name)' 
           :data-bs-parent='"#mod-details-"+fitTextToAttribute(mod.name)'>
        <div class='accordion-body'>
          <p class='mod-description'>{{ modDescription }}</p>
          <div :id='"dependencies-"+mod.name' class='dependencies'>
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
     * Stop mouse click event from propagating up to the parent accordion div and opening it.
     */
    doNotOpenAccordion: function(event: MouseEvent) {
      event.stopPropagation();
      event.preventDefault();
      event.stopImmediatePropagation();
    },

    /**
     * Either enables or disables a mod depending on the mod's current enabled status.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    enableOrDisableMod: async function(event: MouseEvent): Promise<void> {
      this.doNotOpenAccordion(event);
      let enableDisableButton = document.getElementById('enable-disable-button-'+
        this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (event.target != enableDisableButton) return;
      await invoke(this.mod?.enabled ? 'disable_mod' : 'enable_mod', 
        { modName: this.mod?.name });
      (this.mod as ModItem).enabled = !this.mod?.enabled;
      enableDisableButton.textContent = this.mod?.enabled ? "Disable" : "Enable";
      const modDetails = document.getElementById('mod-details-'+this.fitTextToAttribute(this.mod?.name as string));
      if (document.getElementById('enabled-mods-tab')?.classList.contains('active') &&
          !this.mod?.enabled) {
        modDetails?.classList.add('d-none');
      } else if (this.mod?.enabled) {
        modDetails?.classList.remove('d-none');
      }
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
      let classAttribute = 'btn btn-success col align-self-center enable-disable-button px-3';
      classAttribute += this.mod?.installed ? '' : ' d-none';
      return classAttribute;
    },

    /**
     * Install a mod.
     * @param {string} modName The name of the mod to be installed
     * @param {string} modVersion The mod's version to be downloaded
     * @param {string} modLink The link to the download of the mod to be installed
     */
    installMod: async function(modName: string, modVersion: string, modLink: string): Promise<void> {
      invoke('install_mod', { modName: modName, modVersion: modVersion, modLink: modLink });
      const progressElement = document.getElementById('current-download-progress') as HTMLDivElement;
      const progressBar = document.getElementById('current-download-progress-bar') as HTMLDivElement;
      progressBar.ariaValueNow = '0';
      var buttons = document.querySelectorAll('.install-uninstall-button, .enable-disable-button');
      buttons.forEach(button => button.setAttribute('disabled', 'true'));
      var current_download_progress = 0;
      progressElement.classList.remove('d-none');
      while (current_download_progress < 100) {
        await invoke('fetch_current_download_progress')
          .then(progress => {
            progressBar.style.width = (progress as string) + '%';
            progressBar.ariaValueNow = progress as string;
            progressBar.innerHTML = (progress as string) + '%';
            current_download_progress = progress as number;
          })
          .catch(e => invoke('debug', { msg: e }));
      }
      progressElement.classList.add('d-none');
      buttons.forEach(button => button.removeAttribute('disabled'));
      const installUninstallButton = document.getElementById('install-uninstall-button'+
        this.fitTextToAttribute(modName)) as HTMLButtonElement;
      const enableDisableButton = document.getElementById('enable-disable-button-'+
        this.fitTextToAttribute(modName)) as HTMLButtonElement;
      enableDisableButton.classList.remove('d-none');
      enableDisableButton.textContent = "Disable";
      installUninstallButton.textContent = "Uninstall";
      const modDetails = document.getElementById('mod-details-'+this.fitTextToAttribute(modName));
      const value = (document.getElementById('mods-search') as HTMLInputElement).value?.toLowerCase() as string;
      if (modName.includes(value)) {
        modDetails?.classList.remove('d-none'); 
      }
      
      // Install dependencies
      const dependencyElement = document.getElementById('dependency-' + this.fitTextToAttribute(modName)) as HTMLUListElement;
      const dependencies = dependencyElement.querySelectorAll('li');
      dependencies.forEach((dep) => {
        invoke('debug', { msg: "Installing dependency of {" + modName + "}: {" + dep.innerText + "}" });
        const modLinkElement = document.getElementById('mod-link-' + this.fitTextToAttribute(dep.innerText)) as HTMLInputElement;
        const modVersionElement = document.getElementById('mod-version-' + this.fitTextToAttribute(dep.innerText)) as HTMLParagraphElement;
        this.installMod(dep.innerText, modVersionElement.innerHTML, modLinkElement.value);
      });
    },

    /**
     * Either installs or uninstalls a mod depending on its current installation status.
     * Also automatically installs the mod's dependencies.
     * @param {MouseEvent} event The mouse event being sent to the button's click handler
     */
    installOrUninstallMod: async function(event: MouseEvent): Promise<void> {
      this.doNotOpenAccordion(event);
      const installUninstallButton = document.getElementById('install-uninstall-button'+
          this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (event.target != installUninstallButton) return;
      const enableDisableButton = document.getElementById('enable-disable-button-'+
          this.fitTextToAttribute((this.mod as ModItem).name)) as HTMLButtonElement;
      if (this.mod?.installed) {
        await invoke('uninstall_mod', { modName: this.mod?.name });
        enableDisableButton.classList.add('d-none');
        if (this.modLink == "") {
          const modDetails = document.getElementById('mod-details-'+this.fitTextToAttribute(this.mod?.name));
          modDetails?.remove();
        } else {
          installUninstallButton.textContent = "Install";
        }

        if (document.getElementById('installed-mods-tab')?.classList.contains('active') ||
            document.getElementById('enabled-mods-tab')?.classList.contains('active')) {
          const modDetails = document.getElementById('mod-details-'+this.fitTextToAttribute(this.mod?.name as string));
          modDetails?.classList.add('d-none');
        }
      } else {
        this.installMod(this.mod?.name as string, this.modVersion as string, this.modLink as string);
        enableDisableButton.classList.remove('d-none');
        enableDisableButton.textContent = "Disable";
        installUninstallButton.textContent = "Uninstall";
      }
      
      (this.mod as ModItem).installed = !this.mod?.installed;
      (this.mod as ModItem).enabled = true;
    },

    /**
     * Update an installed mod to the most recent version on modlinks.
     */
    updateMod: function(): void {
      const updateModButton = document.getElementById('update-button-' +
        this.fitTextToAttribute(this.mod?.name as string)) as HTMLButtonElement;
      updateModButton.classList.add('d-none');
      this.installMod(this.mod?.name as string, this.modVersion as string, this.modLink as string);
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
