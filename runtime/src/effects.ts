/**
 * Client-side effect replay — computed, debounce, and registered side effects.
 */

import { signalId, type SignalCell, type RawSignalId } from "./signals.js";
import type { ResumaGlobal } from "./core.js";

export interface ClientEffectSpec {
  id: number;
  deps: RawSignalId[];
  captures?: Record<string, RawSignalId>;
  kind: string;
  body: string;
  target?: RawSignalId;
  debounce_ms?: number;
}

function buildEffectState(
  captures: Record<string, RawSignalId> | undefined,
  signals: Map<string, SignalCell<unknown>>,
  global: ResumaGlobal,
): Record<string, SignalCell<unknown>> {
  const local: Record<string, SignalCell<unknown>> = Object.create(global.state);
  if (!captures) return local;
  for (const [name, idRaw] of Object.entries(captures)) {
    const cell = signals.get(signalId(idRaw));
    if (cell) local[name] = cell;
  }
  return local;
}

export function initEffects(
  effects: ClientEffectSpec[],
  signals: Map<string, SignalCell<unknown>>,
  global: ResumaGlobal,
): void {
  for (const eff of effects) {
    try {
      const state = buildEffectState(eff.captures, signals, global);
      const run = new Function("state", "__resuma", eff.body) as (
        state: Record<string, SignalCell<unknown>>,
        resuma: ResumaGlobal,
      ) => void;

      const schedule = () => {
        try {
          run(state, global);
        } catch (err) {
          console.error("[resuma] effect", eff.id, err);
        }
      };

      schedule();

      for (const dep of eff.deps) {
        const cell = signals.get(signalId(dep));
        cell?.subscribe(() => schedule());
      }
    } catch (err) {
      console.error("[resuma] effect init", eff.id, err);
    }
  }
}
