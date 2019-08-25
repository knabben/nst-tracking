export const PRODUCT = '[Product]';

// action types
export const REGISTER_PRODUCT = `${PRODUCT} REGISTER`;
export const SET_PRODUCT = `${PRODUCT} SET`;

// action creators
export const registerProduct = (values, token) => ({
  type: REGISTER_PRODUCT,
  payload: values,
  meta: {token}
})

export const setProduct = ({product}) => ({
  type: SET_PRODUCT,
  payload: product,
});