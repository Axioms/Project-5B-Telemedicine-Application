import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import vuetify from "./plugins/vuetify";
import firebase from "firebase/compat/app";
import "firebase/compat/auth"
import "firebase/compat/firestore";

const firebaseConfig = {
  apiKey: process.env.VUE_APP_FIREBASE_API_KEY,
  authDomain: process.env.VUE_APP_FIREBASE_AUTH_DOMAIN,
  projectId: process.env.VUE_APP_FIREBASE_PROJECT_ID,
  storageBucket: process.env.VUE_APP_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: process.env.VUE_APP_FIREBASE_MESSAGING_SENDER_ID,
  appId: process.env.VUE_APP_FIREBASE_APP_ID
};

// Initialize Firebase
// Initialize Firebase
const firebaseApp = firebase.initializeApp(firebaseConfig);
const timestamp = firebase.firestore.FieldValue.serverTimestamp();

export{timestamp}
export default firebaseApp.firestore();


Vue.config.productionTip = false;

let app: any;


    firebase.auth().onAuthStateChanged(() =>{
      if(!app) {
        new Vue({
          router,
          store,
          vuetify,
          render: (h) => h(App),
        }).$mount("#app");
      }
    });



/*
new Vue({
  router,
  store,
  vuetify,
  render: (h) => h(App),
}).$mount("#app");
*/