// Add on-click data select and dropdown close on children
document.querySelectorAll(".choices__item--choice.choices__item--selectable").forEach((e) => {

    // Create listener
    e.addEventListener("click", (event) => {

        // Get real and shown object
        let visibleElements = event.target.parentElement.parentElement.parentElement.children[0].children;
    
        // Set real value
        visibleElements[0].children[0].innerHTML = event.target.innerHTML;
        visibleElements[0].children[0].value = e.dataset.value;

        // Set shown value
        visibleElements[1].children[0].innerHTML = event.target.innerHTML;

        // Trigger onchange events
        let onchange = visibleElements[0].dataset.onchange;
        if (onchange !== undefined) {
            window[onchange](event.target.innerHTML, e.dataset.value);
        }

        // Close dropdown
        event.target.parentElement.parentElement.classList.remove("is-active");
    });
    
});

// Add on-click dropdown open to parents
document.querySelectorAll(".choices__inner").forEach((e) => {

    // Create listener
    e.addEventListener("click", () => {

        // Open dropdown
        e.parentElement.children[1].classList.toggle("is-active");
    });

});

// Add blur event to close dropdown after clicking somewhere else
document.querySelectorAll(".choices").forEach((e) => {

    // Create listener
    e.addEventListener("blur", () => {

        // Close dropdown
        e.children[1].classList.remove("is-active");
    });

});