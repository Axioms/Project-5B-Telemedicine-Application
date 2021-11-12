<template src="./html/provider-portal.html"></template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';
import VueRouter from 'vue-router';
import store from 'vuex';

@Options({
  components: {
    
  },
})
export default class ProviderPortal extends Vue {
    mounted() {
        this.$store.dispatch('setIsLoggedIn', true);
        this.$store.dispatch('setIsProvider', true);
    }

}
</script>
