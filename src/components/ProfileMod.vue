<template>
  <li class='form-check profile-mod'>
      <input :id='"profile-mod-"+modName' 
             class='form-check-input profile-mod-checkbox' 
             type='checkbox' 
             value='' />
      <label class='form-check-label profile-mod-label' :for='"profile-mod-"+modName'>
        {{ modName }}
      </label>
  </li>
</template>

<script lang='ts'>
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'ProfileMod',
  props: {
    modName: String,
  }
});
</script>