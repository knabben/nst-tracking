import { BID, REGISTER_BID, FETCH_BID } from '../actions/bid'
import { apiRequest } from '../actions/api';
import { setLoader } from '../actions/ui';

const BID_URL = 'http://localhost:8086/api/bid';

export const bidMiddleware = ({dispatch}) => (next) => (action) => {
  next(action);

  switch (action.type) {
    case FETCH_BID:
      next(apiRequest({method: 'GET', url: BID_URL, token: action.meta.token, feature: BID}))
      next(setLoader({state: true, feature: BID}))
      break;

    case REGISTER_BID:
      next(apiRequest({body: action.payload,token: action.meta.token,  method: 'POST', url: BID_URL, feature: BID}))
      next(setLoader({state: true, feature: BID}))
      break;
  }
}