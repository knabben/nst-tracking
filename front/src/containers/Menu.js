import Menu from '../components/Menu'
import { isAuthenticated } from '../selectors/user'
import { connect } from 'react-redux'

const mapStateToProps = (state) => ({
  authenticated: isAuthenticated(state),
})

const MenuContainer = connect(mapStateToProps)(Menu);
export default MenuContainer;