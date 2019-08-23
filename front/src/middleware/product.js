import { push } from 'connected-react-router'

import { PRODUCT, REGISTER_PRODUCT, registerProduct } from '../actions/product'
import { API_SUCCESS, API_ERROR, apiRequest } from '../actions/api';

import { setLoader } from '../actions/ui';
import { setNotification } from '../actions/notification'

const PRODUCT_URL = 'http://localhost:8086/api/product';

export const productMiddleware = ({dispatch}) => (next) => (action) => {
  next(action);

  switch (action.type) {
    case REGISTER_PRODUCT:
      next(apiRequest({body: action.payload, method: 'POST', url: PRODUCT_URL, feature: PRODUCT}))
      next(setLoader({state: true, feature: PRODUCT}))
      dispatch(push('/home'));
      break;
  }
};