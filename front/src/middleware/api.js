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
      .then(response => {
        const data = response.json()
        if (response.status == 200) {
          return data
        } else {
          dispatch(apiError(data, feature))
        }
      })
      .then(data => dispatch(apiSuccess(data, feature)))
      .catch(error => dispatch(apiError(error, feature)))
  }
};