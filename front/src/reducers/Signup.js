const login = (state = [], action) => {
  switch (action.type) {
    case "SUCCESS_REGISTER":
      return {
          token: action.payload.token
      }
    default:
      return state;
  }
}

export default login;