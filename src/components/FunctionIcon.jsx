import { useState } from "react";

function FunctionIcon({ func, selected, onClick }) {
  const [showLabel, setShowLabel] = useState(false);
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div
      className="relative"
      onMouseEnter={() => {
        setShowLabel(true);
        setIsHovered(true);
      }}
      onMouseLeave={() => {
        setShowLabel(false);
        setIsHovered(false);
      }}
    >
      <button
        onClick={() => onClick(func.name)}
        className={`h-10 w-[50px] rounded-r-[20px] flex items-center justify-center cursor-pointer
          transition-transform duration-300 ease-in active:translate-y-0.5
          ${selected 
            ? "bg-gradient-to-br from-[#87E9A4] via-[#ADC5FF] to-[#00FFFF] animate-gradient bg-[length:200%_200%]" 
            : "bg-[#B0E0E6] hover:bg-[#9FD5DB]"
          }`}
      >
        <img 
          src={`/assets/fun-icon/${func.name}.svg`}
          alt={func.describe}
          className="w-9 h-9 ml-0.5 mt-0.5 transition-transform duration-1000"
          style={{ transform: isHovered ? 'rotateY(360deg)' : 'rotateY(0deg)' }}
        />
      </button>
      {showLabel && (
        <div className="absolute left-[55px] top-1/2 -translate-y-1/2 bg-black/80 text-white 
          px-2 py-1 rounded text-xs whitespace-nowrap z-50 pointer-events-none shadow-md">
          {func.describe}
        </div>
      )}
    </div>
  );
}

export default FunctionIcon;
