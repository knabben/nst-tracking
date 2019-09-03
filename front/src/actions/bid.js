export const BID = '[Bid]';

// action types
export const REGISTER_BID = `${BID} REGISTER`;
export const FETCH_BID = `${BID} FETCH`;
export const SET_BID = `${BID} SET`;
export const TRANSFER_BID = `${BID} TRANSFER`;

// action creators
export const fetchBid = (token) => ({
  type: FETCH_BID,
  meta: {token}
})

export const registerBid = (values, token) => ({
  type: REGISTER_BID,
  payload: values,
  meta: {token}
})

export const setBid = ({bid}) => ({
  type: SET_BID,
  payload: bid,
});

