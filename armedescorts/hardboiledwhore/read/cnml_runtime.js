// CNML Web Component Runtime (Optimized O(n) traversal)
// Generated automatically

const DIMS = ["amp","arousal","force","tex","att","chan","sal"];

const RESOLUTION = {
  amp: "override",
  arousal: "override",
  force: "override",
  tex: "override",
  att: "override",
  chan: "override",
  sal: "add_clamp"
};

const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));

function getContribution(el, dim) {
  if (el.dataset && el.dataset[dim] !== undefined) {
    const n = Number(el.dataset[dim]);
    if (Number.isFinite(n)) return n;
  }
  const cs = getComputedStyle(el);
  const raw = cs.getPropertyValue(`--${dim}`).trim();
  if (!raw) return null;
  const n = Number(raw);
  return Number.isFinite(n) ? n : null;
}

function nextMode(el, inherited) {
  const tag = el.tagName;
  if (tag === "CNML-NONACTUAL") return "nonactual";
  if (tag === "CNML-NARRATIVE") return "narrative";
  return inherited;
}

function nextProsody(el, prev) {
  const state = { ...prev };

  for (const dim of DIMS) {
    const c = getContribution(el, dim);
    if (c === null) continue;

    if (RESOLUTION[dim] === "override") {
      state[dim] = c;
    } else if (RESOLUTION[dim] === "add_clamp") {
      state[dim] = clamp(state[dim] + c, -1, 1);
    }
  }
  return state;
}

function applyAll(el, state, mode) {
  el.setAttribute("cnml-resolved", "");
  for (const dim of DIMS) {
    const v = state[dim] ?? 0;
    el.style.setProperty(`--${dim}-final`, v);
  }
  el.dataset.mode = mode;
  el.style.setProperty("--mode", mode);
}

function walk(node, inheritedState, inheritedMode) {
  if (node.nodeType !== 1) return;

  const mode = nextMode(node, inheritedMode);
  const state = nextProsody(node, inheritedState);

  applyAll(node, state, mode);

  const children = node.children;
  for (let i = 0; i < children.length; i++) {
    walk(children[i], state, mode);
  }
}

class CNMLElement extends HTMLElement {
  connectedCallback() {
    this.updateTree();
  }

  updateTree() {
    const rootState = {
      amp: null,
      arousal: null,
      force: null,
      tex: null,
      att: null,
      chan: null,
      sal: 0
    };
    const rootMode = "narrative";
    walk(this, rootState, rootMode);
  }
}

const TAGS = [
  "cnml-book","cnml-act","cnml-chapter","cnml-scene","cnml-poem",
  "cnml-nonactual","cnml-narrative",
  "cnml-say","cnml-think","cnml-feel","cnml-gesture","cnml-shout",
  "cnml-quiet","cnml-loud",
  "cnml-calm","cnml-excited",
  "cnml-gentle","cnml-forceful",
  "cnml-emph","cnml-deemph",
  "cnml-smooth","cnml-harsh",
  "cnml-engaged","cnml-detached",
  "cnml-nonverbal"
];

for (const t of TAGS) {
  if (!customElements.get(t)) {
    customElements.define(t, CNMLElement);
  }
}

const mo = new MutationObserver(() => {
  document.querySelectorAll(TAGS.join(",")).forEach(el => {
    if (el.updateTree) el.updateTree();
  });
});

mo.observe(document.documentElement, {
  childList: true,
  subtree: true,
  attributes: true
});
