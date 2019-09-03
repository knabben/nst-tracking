import { SET_BID } from "../actions/bid";

const initState = [];

export const bidReducer = (bid = initState, action) => {
  switch (action.type) {
    case SET_BID:
      return action.payload
    
    default:
      return bid
  }
}