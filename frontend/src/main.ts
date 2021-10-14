import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import vuetify from "./plugins/vuetify";
import firebase from "firebase/compat/app";
import "firebase/compat/auth"
import "firebase/compat/firestore";

const firebaseConfig = {
  apiKey: "***REMOVED***",
  authDomain: "***REMOVED***",
  projectId: "***REMOVED***",
  storageBucket: "***REMOVED***.appspot.com",
  messagingSenderId: "***REMOVED***",
  appId: "1:***REMOVED***:web:b5a31d686b40e7eb77f353"
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