import { push } from 'connected-react-router';
import { API_SUCCESS } from '../actions/api'

export const routerDispatchMiddleware = ({dispatch}) => (next) => (action) => {
  if (action.type.includes(API_SUCCESS)) {
    dispatch(push('/home'));
  }
  next(action);
}