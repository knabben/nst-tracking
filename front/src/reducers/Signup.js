const login = (state = [], action) => {
  switch (action.type) {
    case 'SUCCESS_REGISTER':
      return [
        ...state,
        {
          token: action.token
        }
      ]
    default:
      return state;
  }
}

export default login;