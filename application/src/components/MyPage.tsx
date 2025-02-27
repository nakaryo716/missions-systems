import { UserInfo } from "@/types/UserInfo";
import { Button, TextField } from "@mui/material";
import style from "@/styles/MyPage.module.css";

type Props = {
  userInfo: UserInfo | null;
  userName: string;
  handleChangeName:
  (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  handleUpdateName: () => Promise<void>;
  handleDelete: () => Promise<void>;
};

export const MyPage = (props: Props) => {
  const {
    userInfo,
    userName,
    handleChangeName,
    handleUpdateName,
    handleDelete,
  } = props;
  return (
    <>
      <div className={style.container}>
        <div className={style.userName}>
          <h2 className={style.title}>User Name</h2>
          <p>{userInfo?.userName}</p>
        </div>
        <div className={style.settings}>
          <h2 className={style.title}>Settings</h2>
          <div>
            <h3>Update user name</h3>
            <div className={style.update}>
              <TextField
                type="text"
                name="user name"
                label="User name"
                variant="filled"
                required
                sx={{ backgroundColor: "white" }}
                value={userName}
                onChange={handleChangeName}
              />
              <Button
                variant="contained"
                color="success"
                sx={{ marginLeft: 2}}
                onClick={handleUpdateName}
              >
                Update
              </Button>
            </div>
          </div>
          <div>
            <h3>Delete account</h3>
            <Button
              variant="contained"
              color="error"
              onClick={handleDelete}
            >
              Delete
            </Button>
          </div>
        </div>
      </div>
    </>
  );
}
