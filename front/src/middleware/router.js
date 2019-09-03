import { push } from 'connected-react-router';
import { PRODUCT } from '../actions/product';
import { API_SUCCESS } from '../actions/api';

export const routerDispatchMiddleware = ({dispatch}) => (next) => (action) => {
  if (action.type.includes(API_SUCCESS) && action.type.includes(PRODUCT)) {
    dispatch(push('/home'));
  }
  next(action);
}