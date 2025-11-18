const WS_URL = `ws://${window.location.host}/ws`;
let ws = null;
let personCount = 0;
let persons = []; // Array to store person data
let walkPersonIndex = 0; // Track which person is walking


window.addEventListener('beforeunload', () => {
    if (ws) {
        ws.close();
    }
});

const statusIndicator = document.getElementById('statusIndicator');
const statusText = document.getElementById('statusText');
const personCountElement = document.getElementById('personCount');
const worldWidthElement = document.getElementById('worldWidth');
const worldHeightElement = document.getElementById('worldHeight');
const timeElapsedElement = document.getElementById('timeElapsed');
let canvas, ctx;
let backgroundImage = null;
// Make backgroundImage globally accessible
window.backgroundImage = null;

// Load background image
const bgImg = new Image();
bgImg.onload = () => {
    backgroundImage = bgImg;
    window.backgroundImage = bgImg; // Make globally accessible
    console.log('Background image loaded');
    // Redraw background if canvas is already configured
    if (canvas) {
        drawBackground();
    }
};
bgImg.onerror = () => {
    console.error('Failed to load background image');
};
bgImg.src = 'img/ozora.png';

// Initialize canvas after DOM is loaded
window.addEventListener('DOMContentLoaded', () => {
    canvas = document.getElementById('personCanvas');
    ctx = canvas.getContext('2d');

    // Initialize canvas with white background
    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    console.log('Initializing WebSocket connection...');
    connect();
});

function drawBackground() {
    if (!canvas || !ctx) return;

    if (backgroundImage) {
        // Draw the background image, scaled to fit the canvas
        ctx.drawImage(backgroundImage, 0, 0, canvas.width, canvas.height);
    } else {
        // Fallback to white background if image not loaded yet
        ctx.fillStyle = 'white';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    }
}

function updateStatus(status, text) {
    statusIndicator.className = `status-indicator ${status}`;
    statusText.textContent = text;
}

function connect() {
    updateStatus('connecting', 'Connecting...');

    try {
        ws = new WebSocket(WS_URL);

        ws.onopen = () => {
            console.log('WebSocket connected');
            updateStatus('connected', 'Connected');
        };

        ws.onmessage = (event) => {
            try {
                const payload = JSON.parse(event.data);
                handleEvent(payload);
            } catch (error) {
                console.error('Failed to parse device data:', error);
            }
        };

        ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            updateStatus('disconnected', 'Connection Error');
        };

        ws.onclose = () => {
            console.log('WebSocket disconnected');
            updateStatus('disconnected', 'Disconnected');

            // Attempt to reconnect after 3 seconds
            setTimeout(() => {
                console.log('Attempting to reconnect...');
                connect();
            }, 3000);
        };
    } catch (error) {
        console.error('Failed to create WebSocket:', error);
        updateStatus('disconnected', 'Connection Failed');
    }
}

function handleEvent(payload) {
    console.log('Received payload:', payload);
    const [action, data] = Object.entries(payload)[0];

    let handlerFn = window[action];
    if (typeof handlerFn !== "function") {
        alert(`Unknown action: ${action}`);
        return;
    }

    handlerFn.call(null, data);
}

