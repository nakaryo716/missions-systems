import style from "@/styles/EditModal.module.css";
import { Box, Button, Modal, TextField } from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";

type Props = {
  open: boolean;
  handleClose: () => void;
  title: string;
  handleSetTitle: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => void;
  description: null | string;
  handleSetDescription: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => void;
  editMissionHandle: (
    title: string,
    description: string | null,
  ) => Promise<void>;
  deleteMissionHandle: () => Promise<void>;
};
const EditModal = (props: Props) => {
  const {
    open,
    handleClose,
    title,
    handleSetTitle,
    description,
    handleSetDescription,
    editMissionHandle,
    deleteMissionHandle,
  } = props;
  return (
    <Modal open={open} onClose={handleClose}>
      <Box sx={modalStyle}>
        <div className={style.input}>
          <TextField
            type="text"
            name="title"
            label="Mission Name"
            fullWidth
            className={style.text}
            variant="filled"
            sx={{ backgroundColor: "white" }}
            value={title}
            onChange={handleSetTitle}
            required
          />
        </div>
        <div className={style.input}>
          <TextField
            type="text"
            id="description"
            name="description"
            label="Description"
            value={description != null ? description : ""}
            onChange={handleSetDescription}
            fullWidth
            multiline
            rows={4}
            variant="filled"
            sx={{ backgroundColor: "white" }}
          />
        </div>
        <div className={style.input}>
          <Button
            variant="contained"
            color="info"
            onClick={() => editMissionHandle(title, description)}
            sx={{ marginRight: 2 }}
          >
            <div className={style.icon}>
              <EditIcon />
            </div>
            Done
          </Button>
          <Button
            variant="contained"
            color="error"
            onClick={deleteMissionHandle}
          >
            <div className={style.icon}>
              <DeleteIcon />
            </div>
            Delete
          </Button>
        </div>
      </Box>
    </Modal>
  );
};

const modalStyle = {
  position: "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  maxWidth: "1000px",
  width: "auto",
  bgcolor: "white",
  border: "2px solid #000",
  boxShadow: 24,
  pt: 2,
  px: 4,
  pb: 3,
};
export default EditModal;
