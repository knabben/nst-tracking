import { combineReducers } from 'redux'
import { connectRouter } from 'connected-react-router'

import { productReducer } from './product'
import { uiReducer } from './ui'
import { userReducer } from './user';
import { notificationsReducer } from './notification';
import { bidReducer } from './bid';

const createRootReducer = (history) => combineReducers({
  router: connectRouter(history),
  bid: bidReducer,
  notification: notificationsReducer,
  product: productReducer,
  user: userReducer,
  ui: uiReducer
});
export default createRootReducer;