import("./pkg")
  .then((wasm) => {
    let canvas = document.getElementById("drawing");
    const ctx_render = canvas.getContext("2d");
    canvas = document.getElementById("nodes");
    const nodes_width = canvas.width;
    const nodes_height = canvas.height;
    const ctx_nodes = canvas.getContext("2d");

    wasm.init();

    let dragging = false;
    let selectedNode = undefined;
    let initialMouseX, initialMouseY;
    canvas.addEventListener("mousedown", (event) => {
      console.log("mousedown");

      const elemLeft = canvas.offsetLeft + canvas.clientLeft;
      const elemTop = canvas.offsetTop + canvas.clientTop;

      const x = event.pageX - elemLeft;
      const y = event.pageY - elemTop;

      wasm.canvas_click(ctx_nodes, x, y);

      selectedNode = wasm.canvas_click_active();
      if (selectedNode != undefined) {
        dragging = true;
        initialMouseX = x;
        initialMouseY = y;
      }
    });
    canvas.addEventListener("mousemove", (event) => {
      console.log("mousemove");

      if (dragging && selectedNode != undefined) {
        const mouseX = event.clientX - canvas.getBoundingClientRect().left;
        const mouseY = event.clientY - canvas.getBoundingClientRect().top;

        wasm.move_node(
          selectedNode,
          mouseX - initialMouseX,
          mouseY - initialMouseY
        );
        wasm.nodes(ctx_nodes, nodes_width, nodes_height);

        initialMouseX = mouseX;
        initialMouseY = mouseY;
      }
    });
    canvas.addEventListener("mouseup", (event) => {
      console.log("mouseup");

      if (dragging && selectedNode != undefined) {
        const mouseX = event.clientX - canvas.getBoundingClientRect().left;
        const mouseY = event.clientY - canvas.getBoundingClientRect().top;

        wasm.move_node(
          selectedNode,
          mouseX - initialMouseX,
          mouseY - initialMouseY
        );
        wasm.nodes(ctx_nodes, nodes_width, nodes_height);
      }

      dragging = false;
      selectedNode = undefined;
    });

    const renderBtn = document.getElementById("render");
    renderBtn.addEventListener("click", () => {
      drawCall(wasm, ctx_render);
      wasm.nodes(ctx_nodes, nodes_width, nodes_height);
    });

    drawCall(wasm, ctx_render);
    wasm.nodes(ctx_nodes, nodes_width, nodes_height);
  })
  .catch(console.error);

const drawCall = (wasm, ctx) => wasm.render(ctx, 250, 250);
