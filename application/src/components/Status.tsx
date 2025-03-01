import style from "../styles/Status.module.css";

type StatusProps = {
  userStatus: Level | null;
};

const Status = ({userStatus}: StatusProps) => {

  if (userStatus == null) {
    return(
      <h1>Failed to get user status</h1>
    )
  }
  return (
    <div className={style.container}>
      <div className={style.data}>
        <h1 className={style.level}>Lv.</h1>
        <h1 className={style.level}>{userStatus.level}</h1>
      </div>
      <div className={style.data}>
        <h2 className={style.title}>Current Exp.</h2>
        <div></div>
        <h2 className={style.num}>{userStatus.experiencePoints}</h2>
      </div>
      <div className={style.data}>
        <h2 className={style.title}>Exp needed to level up&emsp;</h2>
        <h2 className={style.num}>{userStatus.remain}</h2>
      </div>
    </div>
  );
}

export default Status;
