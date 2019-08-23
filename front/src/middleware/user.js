import { USER, LOGIN_USER, REGISTER_USER, setUser } from '../actions/user'
import { API_SUCCESS, API_ERROR, apiRequest } from '../actions/api';
import { setLoader } from '../actions/ui';
import { setNotification } from '../actions/notification'

const LOGIN_AGENT_URL = 'http://localhost:8086/api/authentication';
const REGISTER_AGENT_URL = 'http://localhost:8086/api/agent';

export const userMiddleware = () => (next) => (action) => {
  next(action);

  switch (action.type) {
    case LOGIN_USER:
      next(apiRequest({body: action.payload, method: 'POST', url: LOGIN_AGENT_URL, feature: USER}))
      next(setLoader({state: true, feature: USER}))
      break;

    case REGISTER_USER:
        next(apiRequest({body: action.payload, method: 'POST', url: REGISTER_AGENT_URL, feature: USER}))
        next(setLoader({state: true, feature: USER}))
        break;
        
    case `${USER} ${API_SUCCESS}`:
      next(setUser({token: action.payload.token}))
      next(setLoader({state: false, feature: USER}))
      break;

    case `${USER} ${API_ERROR}`:
      next(setNotification({message: action.payload.message, feature: USER}))
      next(setLoader({state: false, feature: USER}))
      break;
  }
}