import { PRODUCT, REGISTER_PRODUCT, FETCH_PRODUCT } from '../actions/product'
import { apiRequest } from '../actions/api';
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
  }
};