import Menu from '../components/Menu'
import { connect } from 'react-redux'

const mapStateToProps = (state) => ({
  authenticated: state.login.hasOwnProperty("token")
})

const MenuContainer = connect(
  mapStateToProps
)(Menu);

export default MenuContainer;