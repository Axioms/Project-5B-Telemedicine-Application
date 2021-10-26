import Vue from "vue";
import Vuex from "vuex";
import createPersistedState from "vuex-persistedstate";
import "firebase/compat/firestore";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"
import { getAuth, updateEmail } from "firebase/auth";
import db from "@/main.ts"

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    isLoggedIn: false,
    isPatient: false,
    isProvider: false,
    profileEmail: null,
    profileFirstName: "",
    profileLastName: "",
    profileID: null,
    profileInitials: "",
    userType: null,
    user: null,
  },
  getters: {
    getIsLoggedIn: state => {
      return state.isLoggedIn;
    },
    getIsPatient: state => {
      return state.isPatient;
    },
    getIsProvider: state => {
      return state.isProvider;
    }
  },
  mutations: {
    updateUser(state, payload)
    {
      state.user = payload;
    },
    setProfileInfo(state, doc)
    {
      state.profileID = doc.id;
      state.profileEmail = doc.data().email;
      state.profileFirstName = doc.data().firstname;
      state.profileLastName = doc.data().lastname;
      state.userType = doc.data().usertype;
    },
    setProfileInitials(context, state)
    {
      state.profileInitials = state.profileFirstName.match(/(\b\S)?/g).join("")+
          state.profileLastName.match(/(\b\S)?/g).join("");
    },
    // method for changing account info on "Account" page
    changeAccountInfo(state, { firstName, lastName, email } ){
      state.profileFirstName = firstName;
      state.profileLastName = lastName;
      state.profileEmail = email;
      
      const currentUser = firebase.auth().currentUser;
      currentUser?.updateEmail(email);

      const dataBase = db.collection("users").doc(currentUser?.uid);
      dataBase.set({
        firstname: firstName,
        lastname: lastName,
        email: email,
      })
    }
  },
  actions: {
    setIsLoggedIn(context, status){
      context.state.isLoggedIn = status;
    },
    setIsPatient(context, status) {
      context.state.isPatient = status;
    },
    setIsProvider(context, status) {
      context.state.isProvider = status;
    },
    async getCurrentUser({commit})
    {
      const database = await db.collection('users').doc(firebase?.auth()?.currentUser?.uid);
      const dbResults = await database.get();
      commit("setProfileInfo", dbResults);
      commit("setProfileInitials");
      console.log(dbResults);
    },
  },
  modules: {},
  plugins: [createPersistedState()]
});
