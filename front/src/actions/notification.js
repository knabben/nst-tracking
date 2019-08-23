export const SET_NOTIFICATION = 'SET_NOTIFICATION';

export const setNotification = ({message, feature}) => ({
  type: `${feature} ${SET_NOTIFICATION}`,
  payload: message,
  meta: {feature}
})