function createRipple(e)
{
  if(this.getElementsByClassName('rippleAnim').length > 0)
    {
      this.removeChild(this.childNodes[1]);
    }
  
  var circle = document.createElement('div');
  this.appendChild(circle);
  
  var d = Math.max(this.clientWidth, this.clientHeight);
  circle.style.width = circle.style.height = d + 'px';
  
  circle.style.left = e.clientX - this.offsetLeft - 10 - d / 2 + 'px';
  circle.style.top = e.pageY - this.offsetTop - d / 2 + 'px';
  
  circle.classList.add('rippleAnim');
}

document.querySelectorAll(".ripple").forEach((e) => {
  e.addEventListener('click', createRipple);
});