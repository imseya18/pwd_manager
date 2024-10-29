// import Login from './pages/login';
// import Password from './password';
// import Vault from './pages/vault';
import Loging from './App'
import SuccessLogin from './SuccessLogin'

import { useState } from "react";
export default function Page() {
    const [isLogin, setIsLogin] = useState(false);

    //return <SelectExample />;
    //return <Vault />;
    //return <Login/>;

    return (
      isLogin ? <SuccessLogin /> : <Loging setIsLogin={setIsLogin} />
    );
    //return <Password/>;
  }
