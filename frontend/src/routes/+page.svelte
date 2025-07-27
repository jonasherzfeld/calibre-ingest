<script lang="ts">
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	
	let fileInput: HTMLInputElement;
	let selectedFile: File | null = null;
	let uploading = false;
	let uploadResult: string | null = null;
	let uploadError: string | null = null;
	let allowedFileTypes: string[] = [];
	let maxFileSizeMB = 25;
	
	const BACKEND_URL = browser ? (window.location.hostname === 'localhost' && window.location.port === '5173' ? 'http://localhost:3000' : '/api') : '';
	
	// Get allowed file types from backend
	onMount(async () => {
		if (browser) {
			try {
				const response = await fetch(`${BACKEND_URL}`);
				if (response.ok) {
					const data = await response.json();
					allowedFileTypes = data.allowed_file_types || [];
					maxFileSizeMB = data.max_file_size_mb || 25;
				}
			} catch (error) {
				console.warn('Could not fetch allowed file types from backend');
				allowedFileTypes = ['epub', 'pdf', 'mobi', 'azw', 'azw3', 'txt'];
			}
		}
	});
	
	function isValidFileType(filename: string): boolean {
		if (allowedFileTypes.length === 0) return true;
		const extension = filename.split('.').pop()?.toLowerCase();
		return extension ? allowedFileTypes.includes(extension) : false;
	}
	
	function isValidFileSize(file: File): boolean {
		return file.size <= maxFileSizeMB * 1024 * 1024;
	}
	
	function handleFileSelect(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0] || null;
		uploadResult = null;
		uploadError = null;
		
		if (file) {
			// Validate file type
			if (!isValidFileType(file.name)) {
				uploadError = `File type not allowed. Allowed types: ${allowedFileTypes.join(', ')}`;
				selectedFile = null;
				target.value = '';
				return;
			}
			
			// Validate file size
			if (!isValidFileSize(file)) {
				uploadError = `File too large. Maximum size is ${maxFileSizeMB}MB.`;
				selectedFile = null;
				target.value = '';
				return;
			}
			
			selectedFile = file;
		} else {
			selectedFile = null;
		}
	}
	
	async function uploadFile() {
		if (!selectedFile) return;
		
		uploading = true;
		uploadResult = null;
		uploadError = null;
		
		try {
			const formData = new FormData();
			formData.append('file', selectedFile);
			
			const response = await fetch(`${BACKEND_URL === '/api' ? '' : BACKEND_URL}/upload`, {
				method: 'POST',
				body: formData
			});
			
			const result = await response.json();
			
			if (response.ok && result.success) {
				uploadResult = `File uploaded successfully: ${result.filename} (${result.size} bytes)`;
				selectedFile = null;
				fileInput.value = '';
			} else {
				uploadError = result.message || 'Upload failed. Please try again.';
			}
		} catch (error) {
			uploadError = 'Network error. Please check your connection and try again.';
		} finally {
			uploading = false;
		}
	}
</script>

<svelte:head>
	<title>Calibre Ingest</title>
	<meta name="description" content="File upload for Calibre library management" />
</svelte:head>

<div class="min-h-screen flex items-center justify-center p-4">
	<div class="bg-white rounded-lg shadow-lg p-8 w-full max-w-md">
		<h1 class="text-3xl font-bold text-center text-gray-800 mb-8">Calibre Ingest</h1>
		
		<div class="space-y-6">
			<div>
				<label for="file-input" class="block text-sm font-medium text-gray-700 mb-2">
					Choose file to upload
				</label>
				<input
					bind:this={fileInput}
					id="file-input"
					type="file"
					on:change={handleFileSelect}
					accept={allowedFileTypes.length > 0 ? allowedFileTypes.map(type => `.${type}`).join(',') : ''}
					class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100 cursor-pointer"
				/>
				{#if allowedFileTypes.length > 0}
					<p class="text-xs text-gray-500 mt-1">
						Allowed types: {allowedFileTypes.join(', ')} • Max size: {maxFileSizeMB}MB
					</p>
				{/if}
			</div>
			
			{#if selectedFile}
				<div class="text-sm text-gray-600 bg-gray-50 p-3 rounded-md">
					<strong>Selected:</strong> {selectedFile.name}<br>
					<strong>Size:</strong> {(selectedFile.size / 1024 / 1024).toFixed(2)} MB
				</div>
			{/if}
			
			<button
				on:click={uploadFile}
				disabled={!selectedFile || uploading}
				class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition duration-200 font-medium"
			>
				{uploading ? 'Uploading...' : 'Upload File'}
			</button>
			
			{#if uploadResult}
				<div class="text-green-600 text-sm bg-green-50 p-3 rounded-md border border-green-200">
					✅ {uploadResult}
				</div>
			{/if}
			
			{#if uploadError}
				<div class="text-red-600 text-sm bg-red-50 p-3 rounded-md border border-red-200">
					❌ {uploadError}
				</div>
			{/if}
		</div>
	</div>
</div>
