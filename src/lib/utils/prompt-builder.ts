export function buildPrompt(
  styleIds: string[],
  schemeIds: string[],
  customPrompt: string | null
): { styles: string[]; color_schemes: string[]; custom_prompt: string | null } {
  return {
    styles: styleIds,
    color_schemes: schemeIds,
    custom_prompt: customPrompt?.trim() || null,
  };
}
