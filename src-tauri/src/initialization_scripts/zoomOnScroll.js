// Event-listener added to the window object to listen to CTRl + scroll events for altering 
// the zoom factor of the webview

function increaseZoomLevel(amount) {
  const percentageString = document.body.style.zoom;
  let num = percentageString === "" ? 100 : parseInt(percentageString.slice(0, percentageString.length - 1));
  let newVal = num + Math.round(amount) < 500 ? num + Math.round(amount) : 500;
  document.body.style.zoom = `${newVal}%`
}

function decreaseZoomLevel(amount) {
  const percentageString = document.body.style.zoom;
  let num = percentageString === "" ? 100 : parseInt(percentageString.slice(0, percentageString.length - 1));
  let newVal = num - Math.round(amount) > 30 ? num - Math.round(amount) : 30;
  document.body.style.zoom = `${newVal}%`
}

window.onkeydown = (ev) => {
  if (ev.key === "Control") {
    window.onwheel = (ev) => {
      if (ev.deltaY > 0) {
        decreaseZoomLevel(10);
      } else if (ev.deltaY < 0) {
        increaseZoomLevel(10);
      }
    }
  }
};

window.onkeyup = (ev) => {
  if (ev.key === "Control") {
    window.onwheel = null;
  }
}