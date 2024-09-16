const positionDataElement = document.getElementById('positionData');
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const ws = new WebSocket('ws://localhost:3001');

ws.onopen = () => {
    console.log('Connected');
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    positionDataElement.innerText = `Position Data: X=${data.x}, Y=${data.y}, Z=${data.z} Pan=${data.cameraPan}`;
    
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    let dotSize;

    // Minimum size of the dot
    if (data.z > 2) {
        dotSize = data.z;
    } else {
        dotSize = 2;
    }

    // Draw a circle based on the players position
    ctx.beginPath();
    ctx.arc(data.x, data.y, dotSize, 0, Math.PI * 2);
    ctx.fillStyle = 'red';
    ctx.fill();
    ctx.closePath();

    const lineLength = 50;
    const angleInRadians = (data.cameraPan - 90) * Math.PI / 180;
    const lineEndX = data.x + lineLength * Math.cos(angleInRadians);
    const lineEndY = data.y + lineLength * Math.sin(angleInRadians);

    // Draw a line towards where the camera is looking
    ctx.beginPath();
    ctx.moveTo(data.x, data.y);
    ctx.lineTo(lineEndX, lineEndY);
    ctx.strokeStyle = 'blue';
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.closePath();
};

ws.onclose = () => {
    console.log('Disconnected');
};