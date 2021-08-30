<template>
  <div class="home">
    <img contain height=300 width=300 alt="Vue logo" src="../assets/logo.png" />
    <HelloWorld msg="This message is being displayed through a Vue class based component!" />
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import HelloWorld from "@/components/HelloWorld.vue"; // @ is an alias to /src

@Options({
  components: {
    HelloWorld,
  },
})
export default class Home extends Vue {}
</script>
