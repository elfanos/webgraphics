const rust = import("./pkg/rust_3d_demo");

// [[3, 2166.98, 2.29, 483.92]
// [2, 2266.71, 0.91, 390.77]
// [12, 2194.84, 2.03, 794.87]
// [7, 2151.17, 3.55, 506.67]
// [10, 2273.5, 2.02, 386.44]
// [6, 2151.17, 1.59, 830.65]
// [5, 2182.07, 3.16, 472.52]
// [8, 2162.83, 1.27, 488.59]
// [9, 2233.78, 5.23, 427.48]
// [11, 2187.75, 4.81, 470.4]
// [1, 2155.34, 1.47, 489.25]
// [4, 2187.12, 1.49, 458.29]
// ]

const testData = [
  [3, 2166.98, 2.29, 483.92],
  [2, 2266.71, 0.91, 390.77],
  [12, 2194.84, 2.03, 794.87],
  [7, 2151.17, 3.55, 506.67],
  [10, 2273.5, 2.02, 386.44],
  [6, 2151.17, 1.59, 830.65],
  [5, 2182.07, 3.16, 472.52],
  [8, 2162.83, 1.27, 488.59],
  [9, 2233.78, 5.23, 427.48],
  [11, 2187.75, 4.81, 470.4],
  [1, 2155.34, 1.47, 489.25],
  [4, 2187.12, 1.49, 458.29],
];

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

    const client = new m.Client();
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
