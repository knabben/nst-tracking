import {API_REQUEST, apiSuccess, apiError} from "../actions/api"

export const apiMiddleware = ({dispatch}) => next => action => {
  next(action);

  if (action.type.includes(API_REQUEST)) {
    const { url, method, feature } = action.meta;

    fetch(url, { 
      method, 
      headers: { 'Content-Type': 'application/json' },
      body:  JSON.stringify(action.payload),
    })
      .then(response => response.json())
      .then(data => dispatch(apiSuccess(data, feature)))
      .catch(error => dispatch(apiError(error, feature)))
  }
};