import {Button, TextField } from "@mui/material";
import AddIcon from "@mui/icons-material/Add"
import style from "../styles/AddMission.module.css";

type AddMissionProps = {
  title: string,
  setTitle: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  description: null | string;
  setDescription: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  submitHandle: () => Promise<void>;
};

const AddMission = (props: AddMissionProps) => {
  const {
    title,
    setTitle,
    description,
    setDescription,
    submitHandle,
  } = props;

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
            value={title}
            onChange={setTitle}
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
            // descriptionがnullの場合は空文字列として扱う
            value={
              description != null ? description : ""
            }
            onChange={setDescription}
          />
        </div>
        <div className={style.input}>
          <Button variant="contained" onClick={submitHandle}>
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
