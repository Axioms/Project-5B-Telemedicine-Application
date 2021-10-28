<template src="../html/patientchat.html">



</template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';

@Options({
  components: {


  },
})
export default class Account extends Vue
{

  messageBox = "";
  chatBox = "Welcome, How can we help you?\n";

  sendMessage ()
  {
    this.chatBox = this.chatBox+this.$store.state.profileFirstName+": "+this.messageBox+"\n";
    this.messageBox = "";


  }


}



</script>


