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

        uploadWorldInput.addEventListener('change', () => {
            let isFileSelected: boolean;
            if (uploadWorldInput.files && uploadWorldInput.files.length > 0) {
                isFileSelected = true;
            } else {
                isFileSelected = false;
            }

            (document.getElementById('world-size') as HTMLInputElement).disabled = isFileSelected;
            (document.getElementById('seed') as HTMLInputElement).disabled = isFileSelected;
            (document.getElementById('generate-seed') as HTMLButtonElement).disabled = isFileSelected;
        });

        const downloadWorldCheckbox = document.getElementById('download-world') as HTMLInputElement;

        downloadWorldCheckbox.addEventListener('change', () => {
            const isDownloadChecked = downloadWorldCheckbox.checked;
            (document.getElementById('robot') as HTMLSelectElement).disabled = isDownloadChecked;
            (document.getElementById('wait') as HTMLInputElement).disabled = isDownloadChecked;
            (document.getElementById('upload-world') as HTMLInputElement).disabled = isDownloadChecked;
            const startButton = document.querySelector('.start-button') as HTMLButtonElement;
            startButton.textContent = isDownloadChecked ? 'Download' : 'Start';
        });

        document.getElementById('generate-form')!.addEventListener('submit', async function (event) {
            event.preventDefault();

            const worldSize = (document.getElementById('world-size') as HTMLInputElement).value;
            const seed = (document.getElementById('seed') as HTMLInputElement).value;
            const wait = (document.getElementById('wait') as HTMLInputElement).value;
            const robot = (document.getElementById('robot') as HTMLSelectElement).value;
            const isDownloadChecked = downloadWorldCheckbox.checked;

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

                const endpoint = isDownloadChecked ? `${BASE_URL}/downloadWorld` : `${BASE_URL}/generate`;

                const response = await fetch(endpoint, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(isDownloadChecked ? {
                        worldSize: formData.worldSize,
                        seed: formData.seed
                    } : formData)
                });

                const data = await response.json();

                if (!data.success) {
                    alert(data.msg); // Display error message
                } else {
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
                }

                // Hide the loading bar
                loadingBarContainer.style.display = 'none';
                if (isDownloadChecked) {
                    return
                }
                // Show notification banner
                const notification = document.createElement('div');
                notification.classList.add('notification');
                notification.textContent = isDownloadChecked ? 'World downloaded successfully' : 'World generated successfully';

                document.body.appendChild(notification);
                // Close notification after 15 seconds
                setTimeout(() => {
                    notification.remove();
                }, 15000);
                // Hide the modal
                const modal = document.getElementById('modal');
                if (modal) {
                    modal.style.display = 'none';

                }
            } catch (error) {
                console.error(error);
                alert('An error occurred while sending data to the server');

                // Hide the loading bar
                const loadingBarContainer = document.getElementById('loading')!;
                loadingBarContainer.style.display = 'none';
            }
            if (!isDownloadChecked) {
                sendCommand(Command.Start);
            }
        });
    });
}
