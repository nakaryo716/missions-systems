import { Button, TextField } from "@mui/material";
import style from "../styles/Login.module.css";
import LoginIcon from '@mui/icons-material/Login';

const Login = () => (
  <div className={style.container}>
    <div className={style.content}>
      <h1>Missions Login</h1>
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
          />
      </div>
      <div className={style.input}>
        <Button variant="contained">
          <div className={style.icon}>
            <LoginIcon />
          </div>
          Login
        </Button>
      </div>
    </div>
  </div>
);

export default Login;
