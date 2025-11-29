import { create } from "zustand";
import { api } from "../api/tauri";

export const useAppStore = create((set, get) => ({
  // State
  text: "",
  selectedFunc: "id-join",
  functions: [],
  skipList: [],

  // Actions
  setText: (text) => set({ text }),
  
  setSelectedFunc: (funcName) => set({ selectedFunc: funcName }),

  loadFunctions: async () => {
    const functions = await api.getFunctions();
    set({ functions });
  },

  loadSkipList: async () => {
    const skipList = await api.getSkipList();
    set({ skipList });
  },

  loadClipboard: async () => {
    const text = await api.getClipboardText();
    set({ text });
  },

  processText: async () => {
    const { text, selectedFunc } = get();
    if (!text.trim()) return;
    
    try {
      const result = await api.processText(text, selectedFunc);
      set({ text: result });
    } catch (err) {
      console.error("Process error:", err);
    }
  },

  hideWindow: () => {
    api.hideWindow();
  },
}));
