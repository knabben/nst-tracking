import { combineReducers } from 'redux'
import { connectRouter } from 'connected-react-router'
import login from './Login'

const createRootReducer = (history) => combineReducers({
  router: connectRouter(history),
  login,
})
export default createRootReducer;