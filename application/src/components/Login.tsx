import { Button, TextField } from "@mui/material";
import style from "../styles/Login.module.css";
import LoginIcon from '@mui/icons-material/Login';
import Link from "next/link";

type LoginProps = {
  emailVal: string;
  handleSetEmail: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  passwordVal: string;
  handleSetPassword: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  handleClickButton: () => void;
  errMsg: string | null;
}

const Login = (props: LoginProps) => {
  const {
    emailVal,
    handleSetEmail,
    passwordVal,
    handleSetPassword,
    handleClickButton,
    errMsg,
  } = props;

  return (
    <div className={style.container}>
      <div className={style.content}>
        <h1>Missions Login</h1>
        <p>{errMsg}</p>
        <div className={style.input}>
          <TextField
            type="email"
            name="title"
            label="Email"
            fullWidth
            className={style.text}
            variant="filled"
            sx={{ backgroundColor: "white" }}
            required
            value={emailVal}
            onChange={handleSetEmail}
          />
        </div>
        <div className={style.input}>
          <TextField
              type="password"
              name="title"
              label="Password"
              fullWidth
              className={style.text}
              variant="filled"
              sx={{ backgroundColor: "white" }}
              required
              value={passwordVal}
              onChange={handleSetPassword}
            />
        </div>
        <div className={style.input}>
          <Button variant="contained" onClick={handleClickButton}>
            <div className={style.icon}>
              <LoginIcon />
            </div>
            Login
          </Button>
        </div>
        <Link href="/signup" style={{color: "#dead2b"}}>サインアップはこちら</Link>
      </div>
    </div>
  );
};

export default Login;
