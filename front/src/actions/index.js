export const SUCCESS_REGISTER = 'SUCCESS_REGISTER';

const CREATE_AGENT = 'http://localhost:8086/api/agent';

function serviceCreateUser(username, password) {
  let data = {"username": username, "password": password};
  return fetch(CREATE_AGENT, {
    method: 'POST', // or 'PUT'
    body: JSON.stringify(data), // data can be `string` or {object}!
    headers:{
      'Content-Type': 'application/json'
    }
  })
}

export function registerUser(username, password) {
  return function(dispatch) {
    return serviceCreateUser(username, password).then(
      response => {
        response.json().then(data =>
          dispatch(sendRegister(data.token))
        )
      },
      error => console.error(error), 
    )
  }
}

export const sendRegister = (token) => ({
    type: SUCCESS_REGISTER,
    payload: {token: token},
})