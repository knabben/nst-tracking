import React from 'react';
import styled from 'styled-components'

import './App.css';
import Button from '@material-ui/core/Button';


const Header = styled.div`
  background-color: red
`

function App() {
  return (
    <div className="App">
      <header className="App-header">

       <Button variant="contained" color="primary">Button</Button>
      </header>
    </div>
  );
}

export default App;
