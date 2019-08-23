import {SET_PRODUCT} from "../actions/product";

const initState = [];

export const productReducer = (product = initState, action) => {
  switch (action.type) {
    case SET_PRODUCT:
      return action.payload
    
    default:
      return product
  }
}