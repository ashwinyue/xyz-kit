import { useEffect, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { useAppStore } from "./store/useAppStore";
import FunctionBar from "./components/FunctionBar";
import EnterButton from "./components/EnterButton";

function App() {
  const textAreaRef = useRef(null);
  
  const {
    text,
    setText,
    selectedFunc,
    setSelectedFunc,
    functions,
    loadFunctions,
    loadClipboard,
    processText,
    hideWindow,
  } = useAppStore();

  useEffect(() => {
    console.log("App mounted, setting up listeners");
    loadFunctions();
    loadClipboard();

    // 监听功能更新事件
    const unlisten = listen("functions-updated", () => {
      console.log("Functions updated, reloading...");
      loadFunctions();
    });

    // 暴露全局函数供后端调用
    window.__reloadClipboard = async () => {
      console.log("=== __reloadClipboard called ===");
      await loadClipboard();
      const currentText = useAppStore.getState().text;
      console.log("clipboard loaded, text:", currentText);
      
      // 直接更新 DOM，不等待 React 状态同步
      if (textAreaRef.current) {
        textAreaRef.current.textContent = currentText;
        console.log("DOM updated with text");
        
        // 使用 setTimeout 确保 DOM 完全更新后再聚焦
        setTimeout(() => {
          if (textAreaRef.current) {
            console.log("focusing and selecting text");
            textAreaRef.current.focus();
            // 选中所有文本
            try {
              const range = document.createRange();
              range.selectNodeContents(textAreaRef.current);
              const selection = window.getSelection();
              selection.removeAllRanges();
              selection.addRange(range);
            } catch (e) {
              console.error("Selection error:", e);
            }
          }
        }, 100);
      }
    };
    
    console.log("Global __reloadClipboard function registered");

    return () => {
      delete window.__reloadClipboard;
      unlisten.then(fn => fn());
    };
  }, [loadFunctions, loadClipboard]);

  // 同步 text 到 contentEditable
  useEffect(() => {
    console.log("text changed:", text);
    if (textAreaRef.current && textAreaRef.current.textContent !== text) {
      console.log("updating contentEditable");
      textAreaRef.current.textContent = text;
    }
  }, [text]);

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === "Escape") {
        hideWindow();
      }
      if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
        processText();
      }
    };

    // 窗口失去焦点时延迟隐藏，避免快捷键唤醒时立即隐藏
    let blurTimer = null;
    const handleBlur = () => {
      blurTimer = setTimeout(() => {
        hideWindow();
      }, 200);
    };

    const handleFocus = () => {
      if (blurTimer) {
        clearTimeout(blurTimer);
        blurTimer = null;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("blur", handleBlur);
    window.addEventListener("focus", handleFocus);
    
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("blur", handleBlur);
      window.removeEventListener("focus", handleFocus);
      if (blurTimer) {
        clearTimeout(blurTimer);
      }
    };
  }, [hideWindow, processText]);

  return (
    <div className="h-screen flex">
      <div
        ref={textAreaRef}
        contentEditable
        suppressContentEditableWarning
        onInput={(e) => setText(e.currentTarget.textContent)}
        className="flex-1 bg-[#f0f8ff] overflow-auto whitespace-pre-wrap break-all p-1.5 
          border-l-2 border-t-2 border-b-2 border-[rgba(3,102,214,0.3)]
          outline-none font-mono"
        style={{ 
          fontFamily: 'JetBrains Mono, monospace',
          borderRadius: '7px 0 7px 7px'
        }}
        onFocus={(e) => {
          e.currentTarget.style.borderColor = '#8070d4';
        }}
        onBlur={(e) => {
          e.currentTarget.style.borderColor = 'rgba(3,102,214,0.3)';
        }}
      />
      <div className="flex flex-col gap-1">
        <FunctionBar
          functions={functions}
          selected={selectedFunc}
          onSelect={(funcName) => {
            setSelectedFunc(funcName);
            processText(); // 点击图标立即执行处理
          }}
        />
      </div>
      <EnterButton onClick={processText} />
    </div>
  );
}

export default App;
