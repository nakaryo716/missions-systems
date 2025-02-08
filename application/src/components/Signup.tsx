import { Button, TextField } from "@mui/material";
import style from "../styles/Login.module.css";
import FlightTakeoffOutlinedIcon from '@mui/icons-material/FlightTakeoffOutlined';

type SignUpProps = {
  userName: string;
  handleSetUserName: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  email: string;
  handleSetEmail: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  password: string;
  handleSetPassword: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
) => void;
  signupHandle: () => Promise<void>;
  err: string | null;
}

const SignUp = (props: SignUpProps) => {
  const {
    userName,
    handleSetUserName,
    email,
    handleSetEmail,
    password,
    handleSetPassword,
    signupHandle,
    err,
  } = props;

  return (
    <div className={style.container}>
      <div className={style.content}>
        <h1>Missions SignUp</h1>
        <p>{err}</p>
        <div className={style.input}>
          <TextField
            type="text"
            name="title"
            label="User name"
            fullWidth
            className={style.text}
            variant="filled"
            sx={{ backgroundColor: "white" }}
            required
            value={userName}
            onChange={handleSetUserName}
          />
        </div>
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
              value={email}
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
              value={password}
              onChange={handleSetPassword}
            />
        </div>
        <div className={style.input}>
          <Button variant="contained" onClick={signupHandle}>
            <div className={style.icon}>
              <FlightTakeoffOutlinedIcon />
            </div>
            SignUp
          </Button>
        </div>
      </div>
    </div>
  );
};

export default SignUp;
