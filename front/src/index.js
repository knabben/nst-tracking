import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import Root from './Router';

import * as serviceWorker from './serviceWorker';
import { BrowserRouter } from 'react-router-dom';
import configureStore from './store'

const store = configureStore()
ReactDOM.render(
  <BrowserRouter>
    <Root store={store} />
  </BrowserRouter>,
  document.getElementById('root')
);

serviceWorker.unregister();
