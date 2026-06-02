/**
 * Tauri Bridge — isolates all Tauri API interactions.
 * Falls back gracefully when running in browser (non-Tauri) mode.
 */

let tauriAvailable = null;

/** Check if running inside Tauri webview */
export function isTauri() {
  if (tauriAvailable === null) {
    tauriAvailable = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;
  }
  return tauriAvailable;
}

/** Dynamically import @tauri-apps/api modules */
async function getTauriCore() {
  if (!isTauri()) return null;
  try {
    const { invoke } = await import(/* @vite-ignore */ "@tauri-apps/api/core");
    const { open } = await import(/* @vite-ignore */ "@tauri-apps/plugin-dialog");
    const { listen } = await import(/* @vite-ignore */ "@tauri-apps/api/event");
    const { getCurrentWindow } = await import(/* @vite-ignore */ "@tauri-apps/api/window");
    const { open: shellOpen } = await import(/* @vite-ignore */ "@tauri-apps/plugin-shell");
    return { invoke, open, listen, getCurrentWindow, shellOpen };
  } catch {
    return null;
  }
}

/**
 * Open a folder dialog and read its contents via Tauri.
 * Returns { folderPath, folderName, entries[] } or null if cancelled.
 * Each entry: { id, name, path, type: "file"|"folder", children? }
 */
export async function tauriOpenFolder() {
  const api = await getTauriCore();
  if (!api) return null;

  const selected = await api.open({ directory: true, multiple: false });
  if (!selected) return null;

  const folderPath = typeof selected === "string" ? selected : selected.path || selected;
  const folderName = folderPath.split(/[/\\]/).filter(Boolean).pop() || "Folder";

  try {
    const entries = await api.invoke("read_directory", { path: folderPath, depth: 5 });
    return { folderPath, folderName, entries };
  } catch (err) {
    console.error("Failed to read directory:", err);
    return null;
  }
}

/**
 * Read a single file's content via Tauri.
 */
export async function tauriReadFile(path) {
  const api = await getTauriCore();
  if (!api) return null;
  try {
    return await api.invoke("read_file", { path });
  } catch (err) {
    console.error("Failed to read file:", err);
    return null;
  }
}

/**
 * Start watching a directory for file changes.
 * onChange callback receives { path, name, kind, watchRoot }.
 * Returns an unlisten function.
 */
export async function tauriWatchFolder(folderPath, onChange) {
  const api = await getTauriCore();
  if (!api) return () => {};

  try {
    await api.invoke("watch_directory", { path: folderPath });
    const unlisten = await api.listen("file-changed", (event) => {
      if (event.payload?.watchRoot === folderPath) {
        onChange(event.payload);
      }
    });
    return async () => {
      unlisten();
      try {
        await api.invoke("unwatch_directory", { path: folderPath });
      } catch {}
    };
  } catch (err) {
    console.error("Failed to watch directory:", err);
    return () => {};
  }
}

/**
 * Save file content via Tauri.
 */
export async function tauriSaveFile(path, content) {
  const api = await getTauriCore();
  if (!api) return false;
  try {
    await api.invoke("save_file", { path, content });
    return true;
  } catch (err) {
    console.error("Failed to save file:", err);
    return false;
  }
}

/** Close the current window */
export async function tauriCloseWindow() {
  const api = await getTauriCore();
  if (!api) return;
  try {
    api.getCurrentWindow().close();
  } catch (err) {
    console.error("Failed to close window:", err);
  }
}

/** Minimize the current window */
export async function tauriMinimizeWindow() {
  const api = await getTauriCore();
  if (!api) return;
  try {
    api.getCurrentWindow().minimize();
  } catch (err) {
    console.error("Failed to minimize window:", err);
  }
}

/** Toggle maximize on the current window */
export async function tauriToggleMaximize() {
  const api = await getTauriCore();
  if (!api) return;
  try {
    api.getCurrentWindow().toggleMaximize();
  } catch (err) {
    console.error("Failed to toggle maximize:", err);
  }
}

/** Start dragging the window — call on mousedown of drag region */
export async function tauriStartDragging() {
  const api = await getTauriCore();
  if (!api) return;
  try {
    await api.getCurrentWindow().startDragging();
  } catch (err) {
    // ignore — non-critical
  }
}

/** Open a URL in the system's default browser */
export async function tauriOpenUrl(url) {
  const api = await getTauriCore();
  if (!api) return;
  try {
    await api.shellOpen(url);
  } catch (err) {
    console.error("Failed to open URL:", err);
  }
}
