const BASE_URL = 'http://0.0.0.0:8080'
window.addEventListener('load', () => {
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

        // Prepare data to send to the backend API
        const formData = {
            worldSize: parseInt(worldSize),
            seed: parseInt(seed),
            wait: parseInt(wait)
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
