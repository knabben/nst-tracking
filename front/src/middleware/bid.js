import { setNotification } from '../actions/notification';
import { BID, REGISTER_BID, FETCH_BID, setBid } from '../actions/bid';
import { apiRequest, API_SUCCESS, API_ERROR } from '../actions/api';
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

    case `${BID} ${API_SUCCESS}`:
      next(setBid({bid: action.payload}))
      next(setLoader({state: false, feature: BID}))
      break;

    case `${BID} ${API_ERROR}`:
      next(setNotification({message: action.payload.message, feature: BID}))
      next(setLoader({state: false, feature: BID}))
      break;
  }
}