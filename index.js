import("./pkg")
  .then((wasm) => {
    const canvas = document.getElementById("nodes");
    const ctx_nodes = canvas.getContext("2d");

    const nodes_width = canvas.width;
    const nodes_height = canvas.height;

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

        console.log({
          mouseX,
          mouseY,
          deltaX: mouseX - initialMouseX,
          deltaY: mouseY - initialMouseY,
        });

        wasm.move_node(
          selectedNode,
          BigInt(mouseX - initialMouseX),
          BigInt(mouseY - initialMouseY)
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
          BigInt(mouseX - initialMouseX),
          BigInt(mouseY - initialMouseY)
        );
        wasm.nodes(ctx_nodes, nodes_width, nodes_height);
      }

      dragging = false;
      selectedNode = undefined;
    });

    wasm.nodes(ctx_nodes, nodes_width, nodes_height);
  })
  .catch(console.error);
