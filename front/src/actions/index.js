export const SEND_REGISTER_USER = 'REGISTER_USER';

function serviceCreateUser(username, password) {
  let data = {"username": username, "password": password};
  return fetch('http://localhost:8086/api/agent', {
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
      response => console.log(response.json()),
      error => console.error(error), 
    )
  }
}

export const sendRegister = (user) => ({
    type: SEND_REGISTER_USER
})