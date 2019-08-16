import { 
  serviceCreateUser,
  serviceLoginUser 
} from '../services'

export const SUCCESS_REGISTER = 'SUCCESS_REGISTER';
export const SUCCESS_LOGIN = 'SUCCESS_LOGIN';

export function registerUser(username, password) {
  return function(dispatch) {
    return serviceCreateUser(username, password).then(
      response => {
        response.json().then(data => dispatch(successRegister(data.token)))
      },
      error => console.error(error), 
    )
  }
}

export function loginUser(username, password) {
  return function(dispatch) {
    return serviceLoginUser(username, password).then(
      response => {
        response.json().then(data => dispatch(successLogin(data.token)))
      },
      error => console.error(error), 
    )
  }
}

export const successRegister = (token) => ({
  type: SUCCESS_REGISTER,
  payload: {token: token},
})

export const successLogin = (token) => ({
  type: SUCCESS_LOGIN,
  payload: { token: token }
})