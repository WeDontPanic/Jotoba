/** Used to create ripple effects on elements containing the class "ripple" */

function createRipple(e) {
  let old = this.getElementsByClassName('rippleAnim');
  if (old.length > 0) {
    this.removeChild(old[0]);
  }

  var circle = document.createElement('div');
  this.appendChild(circle);

  var d = Math.max(this.clientWidth, this.clientHeight);
  var halfX = this.clientWidth >= this.clientHeight * 2;

  circle.style.width = circle.style.height = d + 'px';

  circle.style.left = e.offsetX - (halfX ? d / 2 : d) + 'px';
  circle.style.top = e.offsetY - d / 2 + 'px';

  circle.style.pointerEvents = "none";

  circle.classList.add('rippleAnim');
}

document.querySelectorAll(".ripple").forEach((e) => {
  e.addEventListener('click', createRipple);
});