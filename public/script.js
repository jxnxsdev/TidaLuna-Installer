document.addEventListener('DOMContentLoaded', () => {
    // DOM Elements
    const releasesContainer = document.getElementById('releases');
    const installBtn = document.getElementById('install-btn');
    const uninstallBtn = document.getElementById('uninstall-btn');
    const statusEl = document.getElementById('status');
    const loaderEl = document.getElementById('loader');
    const toastEl = document.getElementById('toast');
    const toastMessageEl = document.getElementById('toast-message');

    let releases = [];
    let selectedReleaseId = null;

    async function fetchReleases() {
        showLoader();
        try {
            const response = await fetch('/releases');
            if (!response.ok) {
                throw new Error('Failed to fetch releases');
            }
            releases = await response.json();
            renderReleases();
            updateStatus('Select a release channel to install');
        } catch (error) {
            console.error('Error fetching releases:', error);
            updateStatus('Failed to load releases. Please try again.', 'error');
        } finally {
            hideLoader();
        }
    }

    function renderReleases() {
        if (!releases.length) {
            releasesContainer.innerHTML = '<p class="no-releases">No release channels available</p>';
            return;
        }

        releasesContainer.innerHTML = '';
        
        releases.forEach((release, index) => {
            const card = document.createElement('div');
            card.className = 'release-card';
            card.dataset.id = release.id;
            
            card.style.animationDelay = `${index * 0.1}s`;
            
            if (release.id === selectedReleaseId) {
                card.classList.add('selected');
            }
            
            card.innerHTML = `
                <div class="release-name">${release.name}</div>
                <div class="release-version">Version ${release.version}</div>
                <div class="release-links">
                    <a href="${release.githubUrl}" class="release-link" target="_blank">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                        </svg>
                        GitHub
                    </a>
                </div>
            `;
            
            card.addEventListener('click', () => selectRelease(release.id));
            releasesContainer.appendChild(card);
        });
    }

    function selectRelease(releaseId) {
        selectedReleaseId = releaseId;
        
        document.querySelectorAll('.release-card').forEach(card => {
            card.classList.remove('selected');
            if (card.dataset.id === releaseId) {
                card.classList.add('selected');
                
                card.style.animation = 'none';
                setTimeout(() => {
                    card.style.animation = 'pulse 2s ease-in-out';
                }, 10);
            }
        });
        
        installBtn.disabled = false;
        
        const selectedRelease = releases.find(r => r.id === releaseId);
        updateStatus(`Ready to install ${selectedRelease.name} (${selectedRelease.version})`);
    }

    async function installRelease() {
        if (!selectedReleaseId) {
            showToast('Please select a release channel first', 'error');
            return;
        }
        
        showLoader();
        updateStatus('Installing TidaLuna...', 'loading');
        
        try {
            const response = await fetch(`/install?release=${selectedReleaseId}`);
            const message = await response.text();
            
            if (response.ok) {
                updateStatus('Installation successful!', 'success');
                showToast('TidaLuna installed successfully!', 'success');
            } else {
                updateStatus(`Installation failed: ${message}`, 'error');
                showToast(`Installation failed: ${message}`, 'error');
            }
        } catch (error) {
            console.error('Error installing TidaLuna:', error);
            updateStatus('Installation failed. Please try again.', 'error');
            showToast('Installation failed. Please try again.', 'error');
        } finally {
            hideLoader();
        }
    }

    async function uninstallTidaLuna() {
        showLoader();
        updateStatus('Uninstalling TidaLuna...', 'loading');
        
        try {
            const response = await fetch('/uninstall');
            const message = await response.text();
            
            if (response.ok) {
                updateStatus('Uninstallation successful!', 'success');
                showToast('TidaLuna uninstalled successfully!', 'success');
                selectedReleaseId = null;
                installBtn.disabled = true;
                renderReleases();
            } else {
                updateStatus(`Uninstallation failed: ${message}`, 'error');
                showToast(`Uninstallation failed: ${message}`, 'error');
            }
        } catch (error) {
            console.error('Error uninstalling TidaLuna:', error);
            updateStatus('Uninstallation failed. Please try again.', 'error');
            showToast('Uninstallation failed. Please try again.', 'error');
        } finally {
            hideLoader();
        }
    }

    function updateStatus(message, type = '') {
        statusEl.textContent = message;
        statusEl.className = 'status';
        if (type) {
            statusEl.classList.add(type);
        }
    }

    function showLoader() {
        loaderEl.classList.remove('hidden');
    }

    function hideLoader() {
        loaderEl.classList.add('hidden');
    }

    function showToast(message, type = '') {
        toastMessageEl.textContent = message;
        toastEl.className = 'toast';
        if (type) {
            toastEl.classList.add(type);
        }
        
        setTimeout(() => {
            toastEl.classList.add('visible');
        }, 10);
        
        setTimeout(() => {
            toastEl.classList.remove('visible');
            setTimeout(() => {
                toastEl.classList.add('hidden');
            }, 300);
        }, 3000);
    }

    function animateBackground() {
        const spheres = document.querySelectorAll('.gradient-sphere');
        spheres.forEach(sphere => {
            sphere.style.animationDelay = `${Math.random() * 5}s`;
        });
    }

    installBtn.addEventListener('click', installRelease);
    uninstallBtn.addEventListener('click', uninstallTidaLuna);

    fetchReleases();
    animateBackground();
});