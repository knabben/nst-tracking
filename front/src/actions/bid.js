export const BID = '[Bid]';

// action types
export const REGISTER_BID = `${BID} REGISTER`;

// action creators
export const registerBid = (values, token) => ({
  type: REGISTER_BID,
  payload: values,
  meta: {token}
})