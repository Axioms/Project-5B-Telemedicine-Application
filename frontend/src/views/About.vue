<template>
  <div class="about">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
      <h1 class="text-center mt-4">About us</h1>
      <v-row class="ma-5" justify="center" >
        The project would build a web portal that can store messages/chat, reports, requests for appts, and somehow host video recordings (our old distance learning Tegrity system could do all that).   I am not up on what HIPPA would require to control access over the net/web, and would require as controls on the repository, in order to be legal.   Perhaps this fall could be phase I, developing the technology, and in Spring a group could lock it down and try to meet HIPPA standards.
        <br />
      </v-row>

      <v-divider class="ma-4"></v-divider>
      
      <a @click="$router.go(-1)" text>
        Go back <v-icon class="ml-3">mdi-arrow-left</v-icon>
      </a>
   </v-card>
  </div>
</template>
