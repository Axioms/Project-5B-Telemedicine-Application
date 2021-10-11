<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <v-app-bar-nav-icon v-if="userIsLoggedIn" @click="drawer = true"></v-app-bar-nav-icon>
      <v-toolbar-title>{{ currentRouteName }}</v-toolbar-title>
      <v-spacer></v-spacer>
      About
      <v-btn class="mr-2" to="/about" icon>
        <v-icon >mdi-information-outline</v-icon>
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer
        v-model="drawer"
        temporary
        absolute
        clipped
        app
      >
        <v-list
          nav
          dense
        >
          <v-list-item-group
            active-class="deep-purple--text text--accent-4"
          >
        <!-- patient navigation options -->
            <v-list-item v-if="isPatient" to="/patientPortal">
              <v-list-item-icon>
                <v-icon>mdi-home</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Home</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isPatient" to="/account">
              <v-list-item-icon>
                <v-icon>mdi-account</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Account</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isPatient" to="/requestAppointments">
              <v-list-item-icon>
                <v-icon>mdi-calendar</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Request an appointment</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isPatient">
              <v-list-item-icon>
                <!-- icon here -->
                <v-icon>mdi-content-save</v-icon>
              </v-list-item-icon>
              <v-list-item-title>View reports</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isPatient">
              <v-list-item-icon>
                <!-- icon here -->
                <v-icon>mdi-calendar</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Calendar</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isPatient">
              <v-list-item-icon>
                <!-- icon here -->
                <v-icon>mdi-chat</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Doctor Chat</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isPatient">
              <v-list-item-icon>
                <!-- icon here -->
                <v-icon>mdi-file-word-outline</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Resources</v-list-item-title>
            </v-list-item>


        <!-- provider navigation options -->
            <v-list-item v-if="isProvider" to="/providerPortal">
              <v-list-item-icon>
                <v-icon>mdi-home</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Home</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isProvider">
              <v-list-item-icon>
                <!-- icon here -->
              </v-list-item-icon>
              <v-list-item-title>Schedule appointment</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isProvider">
              <v-list-item-icon>
                <!-- icon here -->
              </v-list-item-icon>
              <v-list-item-title>View reports</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isProvider">
              <v-list-item-icon>
                <!-- icon here -->
              </v-list-item-icon>
              <v-list-item-title>Calendar</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isProvider">
              <v-list-item-icon>
                <!-- icon here -->
              </v-list-item-icon>
              <v-list-item-title>Schedule a call</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isProvider">
              <v-list-item-icon>
                <!-- icon here -->
              </v-list-item-icon>
              <v-list-item-title>Patient Chat</v-list-item-title>
            </v-list-item>

            <v-list-item v-if="isProvider">
              <v-list-item-icon>
                <!-- icon here -->
              </v-list-item-icon>
              <v-list-item-title>Education</v-list-item-title>
            </v-list-item>

          </v-list-item-group>
        </v-list>
      </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import "firebase/compat/firestore";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"

export default Vue.extend({
  name: "App",

  data: () => ({
    drawer: false
  }),
  computed: {
    currentRouteName() {
        return this.$route.name;
    },
    userIsLoggedIn() {
      return this.$store.getters.getIsLoggedIn;
    },
    isPatient() {
      return this.$store.getters.getIsPatient;
    },
    isProvider() {
      return this.$store.getters.getIsProvider;
    }
  }
});
</script>
