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
      <div class = "profile" ref ="profile">
        <span>
          {{this.$store.state.profileFirstName+" "+this.$store.state.profileLastName}}
        </span>
      </div>

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

          <v-list-item v-if="isPatient" to="/patientReports">
            <v-list-item-icon>
              <!-- icon here -->
              <v-icon>mdi-content-save</v-icon>
            </v-list-item-icon>
            <v-list-item-title>View reports</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isPatient" to="/patientCalendar">
            <v-list-item-icon>
              <!-- icon here -->
              <v-icon>mdi-calendar</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Calendar</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isPatient" to="/docChat">
            <v-list-item-icon>
              <!-- icon here -->
              <v-icon>mdi-chat</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Doctor Chat</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isPatient" to="/resources">
            <v-list-item-icon>
              <!-- icon here -->
              <v-icon>mdi-file-word-outline</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Resources</v-list-item-title>

          </v-list-item>

          <v-list-item v-if="isPatient" @click="signOut">
            <v-list-item-icon>
              <!-- icon here -->
              <v-icon>mdi-logout</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Log out</v-list-item-title>

          </v-list-item>



          <!-- provider navigation options -->
          <v-list-item v-if="isProvider" to="/providerPortal">
            <v-list-item-icon>
              <v-icon>mdi-home</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Home</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="/account">
            <v-list-item-icon>
              <v-icon>mdi-account</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Account</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="/scheduleAppointment">
            <v-list-item-icon>
              <v-icon>mdi-calendar-clock</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Schedule appointment</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="/providerReports">
            <v-list-item-icon>
              <v-icon>mdi-file</v-icon>
            </v-list-item-icon>
            <v-list-item-title>View reports</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="/providerCalendar">
            <v-list-item-icon>
              <v-icon>mdi-calendar</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Calendar</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="/scheduleCall">
            <v-list-item-icon>
              <v-icon>mdi-phone</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Schedule a call</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="patientChat">
            <v-list-item-icon>
              <v-icon>mdi-chat</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Patient Chat</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isProvider" to="/education">
            <v-list-item-icon>
              <v-icon>mdi-book</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Education</v-list-item-title>
          </v-list-item>


          <v-list-item v-if="isProvider" @click="signOut">
            <v-list-item-icon>
              <!-- icon here -->
              <v-icon>mdi-logout</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Log out</v-list-item-title>

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
    },

  },
  created() {
    firebase.auth().onAuthStateChanged((user) =>
    {
      this.$store.commit("updateUser", user);
      if(user)
      {
        this.$store.dispatch("getCurrentUser");
        console.log(this.$store.state.profileEmail);
      }
    })
  },
  methods:
      {
        async signOut()
        {
          await firebase.auth().signOut();
          await this.$router.push('/')
          await location.reload();

        }
      }


});
</script>
