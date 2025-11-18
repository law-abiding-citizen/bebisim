function ConfigureWorld(config) {
    const canvas = document.getElementById('personCanvas');
    canvas.width = config.width;
    canvas.height = config.height;

    // Update world dimensions display
    worldWidthElement.textContent = config.width;
    worldHeightElement.textContent = config.height;

    // Draw background image (or white if not loaded yet)
    drawBackground();

    console.log(`World configured: ${config.width}x${config.height}`);
}

function AddPerson(person) {
    personCount++;
    personCountElement.textContent = personCount;

    // Store person data
    persons.push({
        name: person.name,
        device: person.device,
        x: person.x,
        y: person.y,
    });

    // Get canvas and context (defined in index.js)
    const canvas = document.getElementById('personCanvas');
    const ctx = canvas.getContext('2d');

    // Draw circle at person's coordinates
    const x = person.x;
    const y = person.y;
    const radius = 6; // Size of the circle

    // Draw solid purple circle
    ctx.fillStyle = '#ff0000'; // Purple color
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fill();

    // Draw person's name next to the circle in white bold
    ctx.fillStyle = 'red';
    ctx.font = 'bold 14px sans-serif';
    ctx.fillText(person.name, x + radius + 5, y + 4);
}

function WalkPerson(position) {
    // Get canvas and context
    const canvas = document.getElementById('personCanvas');
    const ctx = canvas.getContext('2d');

    // Update person position (WalkPerson is sent in the same order as persons were added)
    if (walkPersonIndex < persons.length) {
        const person = persons[walkPersonIndex];
        const radius = 6;
        const oldX = person.x;
        const oldY = person.y;

        // Clear old position - draw a white rectangle (background will show through)
        const textWidth = ctx.measureText(person.name).width;
        const clearWidth = Math.max(radius * 2 + textWidth + 15, 100);
        const clearHeight = radius * 2 + 25;

        // Clear old position by redrawing background
        // Get backgroundImage from global scope (defined in index.js)
        const bgImg = window.backgroundImage || backgroundImage;
        if (bgImg && bgImg.complete) {
            // Calculate source coordinates for the background image (accounting for scaling)
            const scaleX = bgImg.width / canvas.width;
            const scaleY = bgImg.height / canvas.height;
            const srcX = (oldX - radius - 5) * scaleX;
            const srcY = (oldY - radius - 10) * scaleY;
            const srcWidth = clearWidth * scaleX;
            const srcHeight = clearHeight * scaleY;

            ctx.drawImage(
                bgImg,
                srcX, srcY, srcWidth, srcHeight,
                oldX - radius - 5, oldY - radius - 10, clearWidth, clearHeight
            );
        } else {
            ctx.fillStyle = 'white';
            ctx.fillRect(oldX - radius - 5, oldY - radius - 10, clearWidth, clearHeight);
        }

        // Update person's position
        person.x = position.x;
        person.y = position.y;

        // Draw person at new position
        ctx.fillStyle = '#ff0000';
        ctx.beginPath();
        ctx.arc(position.x, position.y, radius, 0, 2 * Math.PI);
        ctx.fill();

        // Draw person's name
        ctx.fillStyle = 'red';
        ctx.font = 'bold 14px sans-serif';
        ctx.fillText(person.name, position.x + radius + 5, position.y + 4);

        // Move to next person for next WalkPerson event
        walkPersonIndex++;

        // Reset index when all persons have been updated (new frame)
        if (walkPersonIndex >= persons.length) {
            walkPersonIndex = 0;
        }
    }
}

function SetTime(seconds) {
    // Update the time display with the number of seconds passed in HH:mm:ss format
    if (timeElapsedElement) {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        const secs = seconds % 60;

        // Format with leading zeros
        const formattedTime =
            String(hours).padStart(2, '0') + ':' +
            String(minutes).padStart(2, '0') + ':' +
            String(secs).padStart(2, '0');

        timeElapsedElement.textContent = formattedTime;
    }
}
