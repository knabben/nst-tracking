import React from 'react';
import { Link } from 'react-router-dom';
import { Container } from '@material-ui/core';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';
import { useStyles } from './styles';
import CssBaseline from '@material-ui/core/CssBaseline';

const Menu = ({authenticated}) => {
  const classes = useStyles();
  const [value, setValue] = React.useState(0)

  function handleChange(event, newValue) {
    setValue(newValue);
  }

  return (
    <Container component="main" maxWidth="xs">
      <CssBaseline />
      
      <div className={classes.menu}>
        <Tabs value={0} onChange={handleChange}>
          { authenticated && 
            <Tab value={0} label="Home" to="/" component={Link}/>
          }
          {
            !authenticated && <div>
              <Tab value={1} label="Signin" to="/signin" component={Link}/>
              <Tab value={2} label="Signup" to="/signup" component={Link}/>
            </div>
          }
        </Tabs>
      </div>
    </Container>
  ) 
}
export default Menu