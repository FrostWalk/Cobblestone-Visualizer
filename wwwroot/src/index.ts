const BASE_URL = 'http://0.0.0.0:8080';

window.addEventListener('load', async () => {
    try {
        // Fetch robots and populate dropdown
        const response = await fetch(`${BASE_URL}/robots`);
        if (!response.ok) {
            throw new Error('Failed to fetch robots from the server');
        }
        const data = await response.json();
        const robots = data.robots; // Estrarre l'array di robot dalla chiave "robots"
        if (!Array.isArray(robots)) {
            throw new Error('Response data is not an array');
        }
        const robotSelect = document.getElementById('robot') as HTMLSelectElement;
        robots.forEach((robot: string) => {
            const option = document.createElement('option');
            option.value = robot;
            option.text = robot;
            robotSelect.add(option);
        });
    } catch (error) {
        console.error(error);
        alert('An error occurred while fetching robots from the server');
    }

    // Show the modal after a short delay
    setTimeout(() => {
        const modal = document.getElementById('modal');
        if (modal) {
            modal.style.display = 'flex';
        }
    }, 200); // Show the modal 1 second after the page loads

    // Handle form submission
    document.getElementById('generate-form')!.addEventListener('submit', async function (event) {
        event.preventDefault();

        const worldSize = (document.getElementById('world-size') as HTMLInputElement).value;
        const seed = (document.getElementById('seed') as HTMLInputElement).value;
        const wait = (document.getElementById('wait') as HTMLInputElement).value;
        const robot = (document.getElementById('robot') as HTMLSelectElement).value;

        // Prepare data to send to the backend API
        const formData = {
            worldSize: parseInt(worldSize),
            seed: parseInt(seed),
            wait: parseInt(wait),
            robot: robot
        };

        try {
            const response = await fetch(`${BASE_URL}/generateWorld`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) {
                throw new Error('Failed to send data to the server');
            }

            // Show notification banner
            const notification = document.createElement('div');
            notification.classList.add('notification');
            notification.textContent = 'Data sent successfully';
            document.body.appendChild(notification);

            // Close notification after 2 seconds
            setTimeout(() => {
                notification.remove();
            }, 2000);

            // Hide the modal
            const modal = document.getElementById('modal');
            if (modal) {
                modal.style.display = 'none';
            }
        } catch (error) {
            console.error(error);
            alert('An error occurred while sending data to the server');
        }
    });

    // Handle "Generate" button click for seed
    document.getElementById('generate-seed')!.addEventListener('click', async function () {
        try {
            const response = await fetch(`${BASE_URL}/randomSeed`);
            if (!response.ok) {
                throw new Error('Failed to fetch random seed from the server');
            }
            const data = await response.json();

            // Update the seed input field with the random seed value
            const seedInput = document.getElementById('seed') as HTMLInputElement;
            seedInput.value = data.seed.toString();
        } catch (error) {
            console.error(error);
            alert('An error occurred while fetching random seed from the server');
        }
    });
});
