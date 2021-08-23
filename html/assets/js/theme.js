/**
 * This JS-File contains the logic for coloring embedded SVG elements
 */

// Execute on load
window.addEventListener("load", colorSvgElements, false);
		
// Function that handles the coloring based on the computed style
function colorSvgElements()
{	
	var style = getComputedStyle(document.body);

	document.querySelectorAll(".svg-embed").forEach((svg, i) => {
		var subdoc = svg.contentDocument;
		subdoc.querySelector("svg").style.fill = style.getPropertyValue(svg.getAttribute("fill"));
		console.log(style.getPropertyValue(svg.getAttribute("fill")));
	});
}
