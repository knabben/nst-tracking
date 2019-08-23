import Menu from '../components/Menu'
import { getToken } from '../selectors/user'
import { connect } from 'react-redux'

const mapStateToProps = (state) => ({
  authenticated: getToken(state),
})

const MenuContainer = connect(mapStateToProps)(Menu);
export default MenuContainer;