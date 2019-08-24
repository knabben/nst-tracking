import { combineReducers } from 'redux'
import { connectRouter } from 'connected-react-router'

import { productReducer } from './product'
import { uiReducer } from './ui'
import { userReducer } from './user';
import { notificationsReducer } from './notification';

const createRootReducer = (history) => combineReducers({
  router: connectRouter(history),
  notification: notificationsReducer,
  product: productReducer,
  user: userReducer,
  ui: uiReducer
});
export default createRootReducer;