/**
 * 工具函数集合
 */

export const formatText = (text) => {
  return text?.trim() || '';
};

export const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch (error) {
    console.error('复制失败:', error);
    return false;
  }
};
