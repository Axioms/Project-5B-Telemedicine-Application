<template src="../html/docchat.html">



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
  chatBox = "Assistant: Welcome, How can we help you?\n";

  sendMessage ()
  {
      this.chatBox = this.chatBox+this.$store.state.profileFirstName+": "+this.messageBox+"\n";
      if(this.messageBox.includes("appointment") ||this.messageBox.includes("calendar"))
      {
        setTimeout(() =>
        {
          this.chatBox = this.chatBox+"Assistant: Head to the schedule appointments and calendar tab to manage your appointments"+"\n";
        },1000);

      }
    else if(this.messageBox.includes("call"))
    {
      setTimeout(() =>
      {
        this.chatBox = this.chatBox+"Assistant: Head to the schedule a call tab to call your doctor"+"\n";
      },1000);
    }
    else
      {
        setTimeout(() =>
        {
          this.chatBox = this.chatBox+"Assistant: I'm not sure how to help with that, try again"+"\n";
        },1000);
      }
      this.messageBox = "";
  }
}



</script>


