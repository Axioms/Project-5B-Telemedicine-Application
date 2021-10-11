import firebase from "firebase/compat/app";
import "firebase/compat/firestore";
// Your web app's Firebase configuration
const firebaseConfig = {
    apiKey: "***REMOVED***",
    authDomain: "***REMOVED***",
    projectId: "***REMOVED***",
    storageBucket: "***REMOVED***.appspot.com",
    messagingSenderId: "***REMOVED***",
    appId: "1:***REMOVED***:web:b5a31d686b40e7eb77f353"
};

// Initialize Firebase
const firebaseApp = firebase.initializeApp(firebaseConfig);
const timestamp = firebase.firestore.FieldValue.serverTimestamp();

export{timestamp}
export default firebaseApp.firestore();