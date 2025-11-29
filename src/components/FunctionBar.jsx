import FunctionIcon from "./FunctionIcon";

function FunctionBar({ functions, selected, onSelect }) {
  return (
    <div className="flex flex-col gap-2">
      {functions.map((func) => (
        <FunctionIcon
          key={func.name}
          func={func}
          selected={selected === func.name}
          onClick={onSelect}
        />
      ))}
    </div>
  );
}

export default FunctionBar;
