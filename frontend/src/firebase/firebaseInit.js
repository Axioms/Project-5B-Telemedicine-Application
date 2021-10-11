import firebase from "firebase/compat/app";
import "firebase/compat/firestore";
// Your web app's Firebase configuration
const firebaseConfig = {
    apiKey: "AIzaSyAfKqd0yI5eI2q7bR5fV1Vw1o-W6ruFIWE",
    authDomain: "telemedicineapp-afe1d.firebaseapp.com",
    projectId: "telemedicineapp-afe1d",
    storageBucket: "telemedicineapp-afe1d.appspot.com",
    messagingSenderId: "817419948074",
    appId: "1:817419948074:web:b5a31d686b40e7eb77f353"
};

// Initialize Firebase
const firebaseApp = firebase.initializeApp(firebaseConfig);
const timestamp = firebase.firestore.FieldValue.serverTimestamp();

export{timestamp}
export default firebaseApp.firestore();