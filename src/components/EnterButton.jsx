import { useState } from "react";

function EnterButton({ onClick }) {
  const [animating, setAnimating] = useState(false);

  const handleClick = async () => {
    setAnimating(true);
    await onClick();
    setTimeout(() => setAnimating(false), 750);
  };

  return (
    <button
      onClick={handleClick}
      title="执行文本处理 (Enter)"
      className={`fixed bottom-5 right-[70px] opacity-85 text-[22px] px-2 py-1 
        bg-[#00CED1] text-white rounded border border-[#00CED1] cursor-pointer
        transition-all duration-100 ease-in
        hover:bg-white hover:text-[#00CED1]
        active:scale-90
        shadow-[0_2px_25px_rgba(72,209,204,0.5)]
        ${animating ? "animate-bubble" : ""}`}
    >
      <span className="font-sans leading-[22px]">Enter</span>
    </button>
  );
}

export default EnterButton;
