// action types
export const REGISTER_PRODUCT = 'REGISTER_PRODUCT';
export const SET_PRODUCT = 'SET_PRODUCT';

// action creators
export const setProduct = ({product}) => ({
  type: SET_PRODUCT,
  payload: product,
});