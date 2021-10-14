import Vue from "vue";
import Vuex from "vuex";
import "firebase/compat/firestore";
import firebase from "firebase/compat/app"
import "firebase/compat/auth"
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
    setProfileInitials(state)
    {
      state.profileInitials = state.profileFirstName.match(/(\b\S)?/g).join("")+
          state.profileLastName.match(/(\b\S)?/g).join("");
    },
   
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
      const database = await db.collection('users').doc(firebase.auth().currentUser.uid);
      const dbResults = await database.get();
      commit("setProfileInfo",dbResults);
      commit("setProfileInitials");
      console.log(dbResults);
    },
  },
  modules: {},
});
