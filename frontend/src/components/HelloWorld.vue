<template>
  <v-container>
    <v-row class="text-center">
      {{ msg }}
    </v-row>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import Options from "vue-class-component";


@Options({
  props: {
    msg: String,
  },
})
export default class HelloWorld extends Vue {
  msg!: string;
}
</script>
