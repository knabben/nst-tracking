import React from 'react';
import SignupForm from '../components/forms/Signup'
import { withFormik } from 'formik';
import { registerUser } from '../actions'
import { connect } from 'react-redux'

const initialValues = {
  username: '',
  password: ''
};

const mapStateToProps = undefined;
const mapDispatchToProps = dispatch => ({
  registerUser(user, password) { 
    dispatch(registerUser(user, password)) 
  },
});

const Signup = connect(
  mapStateToProps, 
  mapDispatchToProps
)(withFormik({
  mapPropsToValues: () => initialValues,
  handleSubmit: (values, actions) => {
    actions.props.registerUser(
      values.username, 
      values.password
    )
  }
})(SignupForm))

export default Signup;