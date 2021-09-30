import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    isLoggedIn: false,
    isPatient: false,
    isProvider: false
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
    }
  },
  modules: {},
});
