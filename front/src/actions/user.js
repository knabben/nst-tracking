// feature name
export const USER = '[User]';

// action types
export const LOGIN_USER = `${USER} LOGIN`; 
export const REGISTER_USER = `${USER} REGISTER`;
export const SET_USER = `${USER} SET`;

// action creators
export const loginUser = ({username, password}) => ({
  type: LOGIN_USER,
  payload: {
    username,
    password
  },
})

export const registerUser = (payload) => ({
  type: REGISTER_USER,
  payload: payload,
})

export const setUser = ({user}) => ({
  type: SET_USER,
  payload: user,
})