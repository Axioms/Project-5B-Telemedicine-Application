<template>
  <div class="login">
    <v-card elevation="4" class="mx-auto mt-5" max-width="40%">
      <v-form ref="form" v-model="validLoginForm" lazy-validation>
        <v-card-title><h2 class="mt-5 mb-5">Log in</h2></v-card-title>

          <v-card-text class="pl-6 pr-6">
            <!-- We should add input validation rules here for username/password -->
            
            <!-- username field -->
            <v-text-field
              v-model="username"
              label="Username"
              required
              outlined
            ></v-text-field>
          
            <!-- password field -->
            <v-text-field
              v-model="password"
              label="Password"
              required
              outlined
            ></v-text-field>
          </v-card-text>

          <v-divider></v-divider>
          
          <v-card-actions>
            <!-- user type selection -->
            <h2>Log in as:</h2>
            <v-radio-group
              v-model="userTypes"
              mandatory
              class="ml-5"
            >
              <v-radio
                label="Patient"
                value="radio-1"
              ></v-radio>
              <v-radio
                label="Provider"
                value="radio-2"
              ></v-radio>
            </v-radio-group>
        
            <v-spacer></v-spacer>

            <!-- login button -->
            <v-btn
              :disabled="!validLoginForm"
              color="success"
              @click="submitLoginForm"
              large
            >
              Log in
            </v-btn>
        </v-card-actions>
      </v-form>
    </v-card>
  </div>
</template>

<script lang="ts">
import Options from "vue-class-component";
import Vue from 'vue';

@Options({
  components: {
    
  },
})
export default class Login extends Vue {
  validLoginForm = true;
  username = '';
  password = '';
  userTypes = '';


  // submitLoginForm (){
    
  // }
}
</script>
