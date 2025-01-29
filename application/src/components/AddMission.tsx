import {Button, TextField } from "@mui/material";
import AddIcon from "@mui/icons-material/Add"
import style from "../styles/AddMission.module.css";

const AddMission = () => {
  return (
    <>
      <div className={style.content}>
        <div className={style.input}>
          <TextField
            type="text"
            name="title"
            label="Mission Name"
            fullWidth
            className={style.text}
            variant="filled"
            sx={{backgroundColor: "white"}}
            required
          />
        </div>
        <div className={style.input}>
          <TextField
            type="text"
            id="description"
            name="description"
            label="Description"
            fullWidth
            multiline
            rows={4}
            variant="filled"
            sx={{backgroundColor: "white"}}
          />
        </div>
        <div className={style.input}>
          <Button variant="contained">
            <div className={style.icon}>
              <AddIcon />
            </div>
            Add Mission
          </Button>
        </div>
      </div>
    </>
  );
};

export default AddMission;
