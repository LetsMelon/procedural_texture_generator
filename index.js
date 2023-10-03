import("./pkg")
  .then((wasm) => {
    let canvas = document.getElementById("drawing");
    const ctx_render = canvas.getContext("2d");
    canvas = document.getElementById("nodes");
    const ctx_nodes = canvas.getContext("2d");

    const renderBtn = document.getElementById("render");

    renderBtn.addEventListener("click", () => {
      drawCall(wasm, ctx_render);
    });

    drawCall(wasm, ctx_render);
    wasm.nodes(ctx_nodes, 500, 500);
  })
  .catch(console.error);

const drawCall = (wasm, ctx) => wasm.render(ctx, 250, 250);
