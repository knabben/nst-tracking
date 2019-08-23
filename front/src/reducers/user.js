import { SET_USER } from "../actions/user";

const initState = [];

export const userReducer = (user = initState, action) => {
  switch (action.type) {
    case action.type.includes(SET_USER):
      return action.payload;

    default:
      return user;
  }
}