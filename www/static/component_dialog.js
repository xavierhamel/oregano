const loadComponentDialog = () => {
    fetch('./components.json')
        .then((value) => value.json())
        .then((components) => {
            let componentSize = 65;
            let dialogContainer = document.querySelector('#dialog-components');
            components.forEach((section) => {
                let label = document.createElement('label');
                label.innerText = section.name;
                let container = document.createElement('div');
                container.classList.add('row', 'dialog-components');
                dialogContainer.appendChild(label);
                dialogContainer.appendChild(container);
                section.components.forEach((component) => {
                    let name = document.createElement('div');
                    name.innerText = component.name;
                    let canvas = document.createElement('canvas');
                    canvas.width = componentSize;
                    canvas.height = componentSize - 20;
                    let componentContainer = document.createElement('div');
                    componentContainer.classList.add('dialog-component-button');
                    componentContainer.appendChild(canvas);
                    componentContainer.appendChild(name);
                    container.appendChild(componentContainer);
                    drawComponent(canvas, component);
                });
            });
        });
};

const drawComponent = (canvas, component) => {
    let ratio = component.ratio || 0.75;
    let offset = component.offset || [0, 0];
    let context = canvas.getContext('2d');
    context.strokeStyle = '#B1B1B1';
    component.icon.polys.forEach((shape) => {
        context.beginPath();
        context.moveTo(
            (shape[0][0] + offset[0]) * ratio,
            (shape[0][1] + offset[1]) * ratio
        );
        for (let idx = 1; idx < shape.length; idx++) {
            let point = shape[idx];
            context.lineTo(
                (point[0] + offset[0]) * ratio,
                (point[1] + offset[1]) * ratio
            );
        }
        context.stroke();
    });
    component.icon.arcs.forEach((arc) => {
        context.beginPath();
        //context.moveTo((arc.x + offset[0] + arc.radius) * ratio, (arc.y + offset[1]) * ratio)
        context.arc(
            (arc.x + offset[0]) * ratio,
            (arc.y + offset[1]) * ratio,
            arc.radius * ratio,
            arc.start * Math.PI / 180,
            arc.end * Math.PI / 180
        );
        context.stroke();
    });
};

