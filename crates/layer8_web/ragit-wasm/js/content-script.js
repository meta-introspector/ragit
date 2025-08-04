// RAGIT Browser Extension Content Script
import init, { RagitWeb, BrowserExtractor } from '../pkg/ragit_wasm.js';

class RagitExtension {
    constructor() {
        this.ragit = null;
        this.extractor = null;
        this.isInitialized = false;
    }

    async initialize() {
        try {
            await init();
            this.ragit = new RagitWeb();
            this.extractor = new BrowserExtractor();
            this.isInitialized = true;
            console.log('RAGIT Extension initialized');
            this.setupKeyboardShortcuts();
        } catch (error) {
            console.error('Failed to initialize RAGIT:', error);
        }
    }

    setupKeyboardShortcuts() {
        document.addEventListener('keydown', (event) => {
            // Ctrl+Shift+R to query
            if (event.ctrlKey && event.shiftKey && event.key === 'R') {
                event.preventDefault();
                this.showQueryInterface();
            }
            // Ctrl+Shift+A to add current page
            if (event.ctrlKey && event.shiftKey && event.key === 'A') {
                event.preventDefault();
                this.addCurrentPage();
            }
        });
    }

    async addCurrentPage() {
        if (!this.isInitialized) return;

        try {
            const content = this.extractor.extract_page_content();
            const title = this.extractor.get_page_title();
            const url = this.extractor.get_page_url();
            
            this.ragit.add_page(content, title, url);
            this.showNotification(`Added: ${title}`);
        } catch (error) {
            console.error('Failed to add page:', error);
        }
    }

    showQueryInterface() {
        if (!this.isInitialized) return;

        const query = prompt('Enter your query:');
        if (query) {
            const results = this.ragit.query(query);
            this.displayResults(results);
        }
    }

    displayResults(results) {
        const modal = this.createResultsModal(results);
        document.body.appendChild(modal);
    }

    createResultsModal(results) {
        const modal = document.createElement('div');
        modal.style.cssText = `
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 80%;
            max-width: 800px;
            max-height: 80%;
            background: white;
            border: 2px solid #333;
            border-radius: 8px;
            padding: 20px;
            z-index: 10000;
            overflow-y: auto;
            box-shadow: 0 4px 20px rgba(0,0,0,0.3);
        `;

        const header = document.createElement('div');
        header.innerHTML = `
            <h3>RAGIT Query Results</h3>
            <button onclick="this.parentElement.parentElement.remove()" style="float: right;">×</button>
        `;
        modal.appendChild(header);

        const resultsList = document.createElement('div');
        results.chunks.forEach(chunk => {
            const item = document.createElement('div');
            item.style.cssText = 'margin: 10px 0; padding: 10px; border: 1px solid #ddd; border-radius: 4px;';
            item.innerHTML = `
                <h4><a href="${chunk.url}" target="_blank">${chunk.title}</a></h4>
                <p>${chunk.content.substring(0, 200)}...</p>
            `;
            resultsList.appendChild(item);
        });
        modal.appendChild(resultsList);

        return modal;
    }

    showNotification(message) {
        const notification = document.createElement('div');
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background: #4CAF50;
            color: white;
            padding: 10px 20px;
            border-radius: 4px;
            z-index: 10001;
        `;
        document.body.appendChild(notification);
        setTimeout(() => notification.remove(), 3000);
    }
}

// Initialize the extension
const ragitExtension = new RagitExtension();
ragitExtension.initialize();