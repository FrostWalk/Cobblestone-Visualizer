import {BASE_URL} from "./variables";
import {sendCommand} from "./websocket";
import {Command} from "./data";

export function addListeners(): void {
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
        }, 200);

        // Handle "Generate seed" button click for seed
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

        // Handle "show advanced" click
        const showAdvanced = document.getElementById('show-advanced');
        const advancedOptions = document.getElementById('advanced-options');
        if (showAdvanced && advancedOptions) {
            showAdvanced.addEventListener('click', () => {
                advancedOptions.style.display = (advancedOptions.style.display === 'none' ? 'block' : 'none');
            });
        }

        const uploadWorldInput = document.getElementById('upload-world') as HTMLInputElement;
        const downloadWorldCheckbox = document.getElementById('download-world') as HTMLInputElement;
        const startButton = document.querySelector('.start-button') as HTMLButtonElement;

        const updateStartButton = () => {
            if (uploadWorldInput.files && uploadWorldInput.files.length > 0) {
                startButton.textContent = 'Upload and Start';
                (document.getElementById('world-size') as HTMLInputElement).disabled = true;
                (document.getElementById('seed') as HTMLInputElement).disabled = true;
                (document.getElementById('download-world') as HTMLInputElement).disabled = true;
                (document.getElementById('generate-seed') as HTMLButtonElement).disabled = true;
                (document.getElementById('wait') as HTMLInputElement).disabled = true;
                (document.getElementById('robot') as HTMLSelectElement).disabled = false;
            } else if (downloadWorldCheckbox.checked) {
                startButton.textContent = 'Download';
                (document.getElementById('robot') as HTMLSelectElement).disabled = true;
                (document.getElementById('wait') as HTMLInputElement).disabled = true;
                (document.getElementById('upload-world') as HTMLInputElement).disabled = true;
            } else {
                startButton.textContent = 'Start';
                (document.getElementById('world-size') as HTMLInputElement).disabled = false;
                (document.getElementById('seed') as HTMLInputElement).disabled = false;
                (document.getElementById('generate-seed') as HTMLButtonElement).disabled = false;
                (document.getElementById('wait') as HTMLInputElement).disabled = false;
                (document.getElementById('robot') as HTMLSelectElement).disabled = false;
            }
        };

        downloadWorldCheckbox.addEventListener('change', updateStartButton);
        uploadWorldInput.addEventListener('change', updateStartButton);

        downloadWorldCheckbox.addEventListener('change', () => {
            const isDownloadChecked = downloadWorldCheckbox.checked;
            (document.getElementById('robot') as HTMLSelectElement).disabled = isDownloadChecked;
            (document.getElementById('wait') as HTMLInputElement).disabled = isDownloadChecked;
            (document.getElementById('generate-seed') as HTMLButtonElement).disabled = isDownloadChecked;
            const startButton = document.querySelector('.start-button') as HTMLButtonElement;
            startButton.textContent = isDownloadChecked ? 'Download' : (uploadWorldInput.files && uploadWorldInput.files.length > 0 ? 'Upload and start' : 'Start');
            (document.getElementById('upload-world') as HTMLInputElement).disabled = isDownloadChecked;
        });

        document.getElementById('generate-form')!.addEventListener('submit', async function (event) {
            event.preventDefault();

            const worldSize = (document.getElementById('world-size') as HTMLInputElement).value;
            const seed = (document.getElementById('seed') as HTMLInputElement).value;
            const wait = (document.getElementById('wait') as HTMLInputElement).value;
            const robot = (document.getElementById('robot') as HTMLSelectElement).value;
            const isDownloadChecked = downloadWorldCheckbox.checked;
            const isFileSelected = uploadWorldInput.files && uploadWorldInput.files.length > 0;
            const formData = {
                worldSize: parseInt(worldSize),
                seed: parseInt(seed),
                wait: parseInt(wait),
                robot: robot
            };

            try {
                // Show the loading bar
                const loadingBarContainer = document.getElementById('loading')!;
                loadingBarContainer.style.display = 'block';

                if (uploadWorldInput.files && uploadWorldInput.files.length > 0) {
                    const file = uploadWorldInput.files[0];
                    const formData = new FormData();
                    formData.append('world', file);
                    formData.append('robot', robot);

                    const response = await fetch(`${BASE_URL}/uploadWorld`, {
                        method: 'POST',
                        body: formData
                    });

                    if (!response.ok) {
                        throw new Error('Failed to upload file to the server');
                    }

                    sendCommand(Command.Start);
                } else if (isDownloadChecked) {
                    const response = await fetch(`${BASE_URL}/downloadWorld`, {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify({
                            worldSize: formData.worldSize,
                            seed: formData.seed
                        })
                    });

                    if (!response.ok) {
                        throw new Error('Failed to send data to the server');
                    }

                    // Download the file using fetch
                    const downloadUrl = `${BASE_URL}/worlds/wall-e_world.zst`;
                    try {
                        const downloadResponse = await fetch(downloadUrl);

                        if (!downloadResponse.ok) {
                            throw new Error(`Failed to download file: ${downloadUrl}`);
                        }

                        const blob = await downloadResponse.blob();
                        const filename = 'wall-e_world.zst'; // Adjust filename if needed

                        const link = document.createElement('a');
                        link.href = URL.createObjectURL(blob);
                        link.download = filename;
                        link.click();
                    } catch (error) {
                        console.error('Error downloading file:', error);
                        alert('Failed to download the file.'); // Inform user about download failure
                    }
                } else {
                    const response = await fetch(`${BASE_URL}/generate`, {
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
                    notification.textContent = 'World generated successfully';
                    document.body.appendChild(notification);

                    // Close notification after 2 seconds
                    setTimeout(() => {
                        notification.remove();
                    }, 10000);

                    // Hide the modal
                    const modal = document.getElementById('modal');
                    if (modal) {
                        modal.style.display = 'none';
                    }
                }

                // Hide the loading bar
                loadingBarContainer.style.display = 'none';

                if (!isDownloadChecked && !isFileSelected) {
                    sendCommand(Command.Start);
                }
            } catch (error) {
                console.error(error);
                alert('An error occurred while sending data to the server');

                // Hide the loading bar
                const loadingBarContainer = document.getElementById('loading')!;
                loadingBarContainer.style.display = 'none';
            }
        });
    });
}
