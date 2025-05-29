import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  return {
    numLevels: await invoke<number>("get_num_levels", {}),
  };
};
