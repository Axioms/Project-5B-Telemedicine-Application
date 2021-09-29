<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <v-app-bar-nav-icon v-if="userIsLoggedIn" @click="drawer = true"></v-app-bar-nav-icon>
      <v-toolbar-title><h2>{{ currentRouteName }}</h2></v-toolbar-title>
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
        v-if="userIsLoggedIn"
      >
        <v-list
          nav
          dense
        >
          <v-list-item-group
            v-model="group"
            active-class="deep-purple--text text--accent-4"
          >
            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-home</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Dashboard</v-list-item-title>
            </v-list-item>
  
            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-calendar-clock</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Appointments</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-content-paste</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Reports</v-list-item-title>
            </v-list-item>
            
            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-message-text-outline</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Messages</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-phone</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Voice Chat</v-list-item-title>
            </v-list-item>
            
            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-help</v-icon>
              </v-list-item-icon>
              <v-list-item-title>FAQ</v-list-item-title>
            </v-list-item>

            <v-list-item>
              <v-list-item-icon>
                <v-icon>mdi-account</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Account</v-list-item-title>
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
      // This needs to be replaced with validation for whether or not a user is logged in (set to true temporarily for testing)
      return true;
    }
  }
});
</script>
