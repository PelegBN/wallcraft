export type View = "home" | "generate" | "preview" | "settings";

let currentView: View = $state("home");

export function getNavigation() {
  return {
    get current() {
      return currentView;
    },
    navigate(view: View) {
      currentView = view;
    },
  };
}
