import { Tab, Tabs } from "@mui/material";
import style from "../styles/Selector.module.css";
import ChecklistRtlOutlinedIcon from '@mui/icons-material/ChecklistRtlOutlined';
import AddTaskOutlinedIcon from '@mui/icons-material/AddTaskOutlined';
import BarChartOutlinedIcon from '@mui/icons-material/BarChartOutlined';

type SelectorProps = {
  value: number,
  handleChange: (e: React.SyntheticEvent<Element, Event>, newValue: number) => void;
};

const Selector = (props: SelectorProps) => {
  const {
    value,
    handleChange,
  } = props;

  return (
    <Tabs className={style.tabs} value={value} onChange={handleChange} textColor="inherit" >
      <Tab label="Missions" className={style.tab} icon={<ChecklistRtlOutlinedIcon />} />
      <Tab label="Add" className={style.tab} icon={<AddTaskOutlinedIcon />} />
      <Tab label="Status" className={style.tab} icon={<BarChartOutlinedIcon />} />
    </Tabs>
  );
};

export default Selector

