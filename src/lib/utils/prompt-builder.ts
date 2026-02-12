import { CATEGORIES } from "./categories";

export function buildPrompt(
  selectedCategoryIds: string[],
  customPrompt: string | null
): { categories: string[]; custom_prompt: string | null } {
  const categoryFragments = selectedCategoryIds
    .map((id) => CATEGORIES.find((c) => c.id === id)?.promptFragment)
    .filter((f): f is string => f !== undefined);

  return {
    categories: categoryFragments,
    custom_prompt: customPrompt?.trim() || null,
  };
}
