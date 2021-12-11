const rust = import("./pkg/rust_3d_demo");

const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", { antialias: true });
rust
  .then((m) => {
    if (!gl) {
      alert("no gl");
      return;
    }

    const FPS_THROTTLE = 1000 / 30;
    var lastDrawItem = -1;

    const client = new m.Client([
      [3, 2145.05, 1.52, 505.79],
      [7, 2145.05, 1.52, 432.79],
      [1, 2145.05, 1.52, 654.79],
    ]);
    const initialTime = performance.now();

    function render() {
      window.requestAnimationFrame(render);
      const currentTime = performance.now();
      if (currentTime > lastDrawItem + FPS_THROTTLE) {
        lastDrawItem = currentTime;
        // rust update call
        if (
          window.innerHeight !== canvas.height ||
          window.innerWidth !== canvas.width
        ) {
          canvas.height = window.innerHeight;
          canvas.clientHeight = window.innerHeight;
          canvas.style.height = window.innerHeight;

          canvas.width = window.innerWidth;
          canvas.clientWidth = window.innerWidth;
          canvas.style.width = window.innerWidth;

          gl.viewport(0, 0, window.innerWidth, window.innerHeight);
        }
        let elapsedTime = currentTime - initialTime;
        client.update(elapsedTime, window.innerHeight, window.innerWidth, [
          [3, 2145.05, 1.52, 505.79],
          [7, 2145.05, 1.52, 432.79],
          [1, 2145.05, 1.52, 654.79],
        ]);
        client.render();
      }
    }
    render();
  })
  .catch(console.error);
