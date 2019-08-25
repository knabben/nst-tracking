export const isAuthenticated = state => state.user.length > 1;
export const getToken = state => state.user;