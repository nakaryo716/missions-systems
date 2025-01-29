import style from "../styles/Status.module.css";

const Status = () => {
  return (
    <div className={style.container}>
      <div className={style.data}>
        <h1 className={style.level}>Lv.</h1>
        <h1 className={style.level}>{8}</h1>
      </div>
      <div className={style.data}>
        <h2 className={style.title}>Current Exp.</h2>
        <div></div>
        <h2 className={style.num}>{10}</h2>
      </div>
      <div className={style.data}>
        <h2 className={style.title}>Next Exp.</h2>
        <h2 className={style.num}>{110}</h2>
      </div>
    </div>
  );
}

export default Status;
