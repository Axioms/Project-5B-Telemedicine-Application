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
    setIsLoggedIn(context, status){
      context.isLoggedIn = status;
    },
    setIsPatient(context, status) {
      context.isPatient = status;
    },
    setIsProvider(context, status) {
      context.isProvider = status;
    }
  },
  actions: {
    
  },
  modules: {},
});
