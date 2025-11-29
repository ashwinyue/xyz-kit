import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const api = {
  getClipboardText: () => invoke("get_clipboard_text"),
  setClipboardText: (text) => invoke("set_clipboard_text", { text }),
  processText: (text, funcName) => invoke("process_text", { text, funcName }),
  getFunctions: () => invoke("get_functions"),
  getSkipList: () => invoke("get_skip_list"),
  setSkipList: (list) => invoke("set_skip_list", { list }),
  hideWindow: () => invoke("hide_window"),
  showWindow: () => invoke("show_window"),
};

export { listen };
