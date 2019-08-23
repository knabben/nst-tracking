import React from 'react';
import { Link } from 'react-router-dom';
import { Container } from '@material-ui/core';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';
import { useStyles } from './styles';
import CssBaseline from '@material-ui/core/CssBaseline';

const Menu = () => {
  const classes = useStyles();
  const [value, setValue] = React.useState(1)
  const [authenticated, setAuthenticated] = React.useState(false)

  return (
    <Container component="main" maxWidth="xs">
      <CssBaseline />
      <div className={classes.menu}>
        <Tabs value={value} onChange={(e, nv) => setValue(nv)}>
          <Tab value={1} label="Signin" to="/signin" component={Link}/>
          <Tab value={2} label="Signup" to="/signup" component={Link}/>
        </Tabs>
      </div>
    </Container>
  ) 
}
export default Menu