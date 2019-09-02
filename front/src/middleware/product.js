import { PRODUCT, REGISTER_PRODUCT, FETCH_PRODUCT, setProduct } from '../actions/product'
import { setNotification } from '../actions/notification'
import { API_ERROR, API_SUCCESS, apiRequest } from '../actions/api';
import { setLoader } from '../actions/ui';

const PRODUCT_URL = 'http://localhost:8086/api/product';

export const productMiddleware = ({dispatch}) => (next) => (action) => {
  next(action);

  switch (action.type) {
    case FETCH_PRODUCT:
      next(apiRequest({method: 'GET', url: PRODUCT_URL, token: action.meta.token, feature: PRODUCT}))
      next(setLoader({state: true, feature: PRODUCT}))
      break;

    case REGISTER_PRODUCT:
      next(apiRequest({body: action.payload, method: 'POST', url: PRODUCT_URL, token: action.meta.token, feature: PRODUCT}))
      next(setLoader({state: true, feature: PRODUCT}))
      break;

    case `${PRODUCT} ${API_SUCCESS}`:
      next(setProduct({product: action.payload}))
      next(setLoader({state: false, feature: PRODUCT}))
      break;
    
    case `${PRODUCT} ${API_ERROR}`:
      next(setNotification({message: action.payload.message, feature: PRODUCT}))
      next(setLoader({state: false, feature: PRODUCT}))
      break;
  }
};