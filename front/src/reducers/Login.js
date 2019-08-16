import { 
  SUCCESS_REGISTER, 
  SUCCESS_LOGIN 
} from "../actions";

const login = (state = [], action) => {
  switch (action.type) {
    case SUCCESS_REGISTER:
      return {
        token: action.payload.token
      }
    case SUCCESS_LOGIN:
      return {
        token: action.payload.token
      }
    default:
      return state;
  }
}

export default login;