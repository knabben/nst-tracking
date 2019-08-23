import { SET_USER } from "../actions/user";

const initState = [];

export const userReducer = (user = initState, action) => {
  switch (action.type) {
    case SET_USER:
      return action.payload;

    default:
      return user;
  }
}