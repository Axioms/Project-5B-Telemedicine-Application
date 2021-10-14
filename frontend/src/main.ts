import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import vuetify from "./plugins/vuetify";
import firebase from "firebase/compat/app";
import "firebase/compat/auth"
import "firebase/compat/firestore";

const firebaseConfig = {
  apiKey: "AIzaSyAfKqd0yI5eI2q7bR5fV1Vw1o-W6ruFIWE",
  authDomain: "telemedicineapp-afe1d.firebaseapp.com",
  projectId: "telemedicineapp-afe1d",
  storageBucket: "telemedicineapp-afe1d.appspot.com",
  messagingSenderId: "817419948074",
  appId: "1:817419948074:web:b5a31d686b40e7eb77f353"
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