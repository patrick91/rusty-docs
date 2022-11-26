"use strict";

/**
 * WARNING!
 *
 * This file is the bridge between the generated Rust binary and the JavaScript interface users call into.
 *
 * This file is not generated.
 *
 * The `index.d.ts` file is also manually maintained, add any new signatures to it.
 */

const internal = require("./bin-package/index.node");

module.exports = {
  get_markdown: internal.get_markdown,
};
