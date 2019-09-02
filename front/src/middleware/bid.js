import { BID, REGISTER_BID } from '../actions/bid'
import { setNotification } from '../actions/notification'
import { API_ERROR, API_SUCCESS, apiRequest } from '../actions/api';
import { setLoader } from '../actions/ui';

const BID_URL = 'http://localhost:8086/api/bid';

export const bidMiddleware = ({dispatch}) => (next) => (action) => {
  next(action);

  switch (action.type) {
    case REGISTER_BID:
      next(apiRequest({body: action.payload, method: 'POST', url: BID_URL, feature: BID}))
      next(setLoader({state: true, feature: BID}))
      break;
  }
}