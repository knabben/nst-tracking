import React from 'react';
import { useSelector } from 'react-redux'
import { Link } from 'react-router-dom';
import { Container } from '@material-ui/core';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';
import { isAuthenticated } from '../selectors/user'
import { useStyles } from './styles';
import CssBaseline from '@material-ui/core/CssBaseline';

const Menu = () => {
  const classes = useStyles();
  const authenticated = useSelector(isAuthenticated);
  const [value, setValue] = React.useState(1);
  
  const onChange = (e, nv) => { setValue(nv) }

  return (
    <Container component="main" maxWidth="xs">
      <CssBaseline />
      <div className={classes.menu}>
        <Tabs value={value} onChange={onChange}>
        { authenticated ? (
          <div>
            <Tab value={0} label="Home" to="/home" component={Link}/>
            <Tab value={1} label="Product" to="/product" component={Link}/>
            <Tab value={1} label="Bids" to="/bids" component={Link}/>
          </div>
        ) : (
          <div>
            <Tab value={0} label="Signin" to="/signin" component={Link}/>
            <Tab value={1} label="Signup" to="/signup" component={Link}/>
          </div>
        )}
        </Tabs>
      </div>
    </Container>
  ) 
}
export default Menu