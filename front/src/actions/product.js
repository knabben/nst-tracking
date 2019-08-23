export const PRODUCT = '[Product]';

// action types
export const REGISTER_PRODUCT = `${PRODUCT} REGISTER`;
export const SET_PRODUCT = `${PRODUCT} SET`;

// action creators
export const registerProduct = (values) => ({
  type: REGISTER_PRODUCT,
  payload: values
})

export const setProduct = ({product}) => ({
  type: SET_PRODUCT,
  payload: product,
});