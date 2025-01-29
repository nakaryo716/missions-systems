import { Avatar } from "@mui/material";
import style from "../styles/Header.module.css"
import TaskAltOutlinedIcon from '@mui/icons-material/TaskAltOutlined';
import Link from "next/link";

const Header = () => {
  return (
    <div className={style.content}>
      <Link href="/">
        <div className={style.logo}>
          <TaskAltOutlinedIcon />
          <h2>Missions</h2>
        </div>
      </Link>
      <div>
        <Link href="/self">
          <Avatar />
        </Link>
      </div>
    </div>
  );
};

export default Header;
