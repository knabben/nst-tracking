import {API_REQUEST, apiSuccess, apiError} from "../actions/api"

export const apiMiddleware = ({dispatch}) => next => action => {
  next(action);

  if (action.type.includes(API_REQUEST)) {
    const { url, method, feature } = action.meta;
    var headers = {'Content-Type': 'application/json'};
    console.log(action.meta)

    if (action.meta.token != "") {
      headers["Authorization"] = `JWT ${action.meta.token}`
    }

    fetch(url, { 
      method, 
      headers: headers,
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