import thunk from 'redux-thunk'
import { createLogger } from 'redux-logger'
import createRootReducer from '../reducers'
import DevTools from '../containers/DevTools'
import { createStore, applyMiddleware, compose } from 'redux'
import { routerMiddleware } from 'connected-react-router'
import { createBrowserHistory } from 'history'

export const history = createBrowserHistory()

const configureStore = preloadedState => {
  const store = createStore(
    createRootReducer(history),
    preloadedState,
    compose(
      applyMiddleware(thunk, routerMiddleware(history), createLogger()),
      DevTools.instrument()
    )
  )

  if (module.hot) {
    // Enable Webpack hot module replacement for reducers
    module.hot.accept('../reducers', () => {
      store.replaceReducer(createRootReducer(history))
    })
  }

  return store
}

export default configureStore