import { SET_NOTIFICATION, setNotification, removeNotification } from '../actions/notification'

export const notificationMiddleware = () => (next) => (action) => {
  if (action.type.includes(SET_NOTIFICATION)) {
    const {payload, meta} = action;
    const id = new Date().getMilliseconds();
    const notification = {id, message: payload}

    next(setNotification({message: notification, feature: meta.feature}));

    setTimeout(() => {
      next(removeNotification({notificationId: id, feature: meta.feature}))
    }, 10000)

  } else {
    next(action);
  }
}