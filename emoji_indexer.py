#!/usr/bin/env python3
"""
RAGIT EMOJI INDEXER - Comprehensive Emoji Analysis System
Searches all files in the ragit codebase for emoji usage and generates detailed report
"""

import os
import re
import json
from collections import defaultdict, Counter
from pathlib import Path

# Emoji regex pattern to match all Unicode emoji characters
EMOJI_PATTERN = re.compile(
    "["
    "\U0001F600-\U0001F64F"  # emoticons
    "\U0001F300-\U0001F5FF"  # symbols & pictographs
    "\U0001F680-\U0001F6FF"  # transport & map symbols
    "\U0001F1E0-\U0001F1FF"  # flags (iOS)
    "\U00002702-\U000027B0"  # dingbats
    "\U000024C2-\U0001F251"  # enclosed characters
    "\U0001F900-\U0001F9FF"  # supplemental symbols
    "\U0001F018-\U0001F270"  # various symbols
    "\U0001F300-\U0001F5FF"  # misc symbols
    "\U0001F600-\U0001F64F"  # emoticons
    "\U0001F680-\U0001F6FF"  # transport
    "\U0001F700-\U0001F77F"  # alchemical
    "\U0001F780-\U0001F7FF"  # geometric shapes extended
    "\U0001F800-\U0001F8FF"  # supplemental arrows
    "\U0001F900-\U0001F9FF"  # supplemental symbols
    "\U0001FA00-\U0001FA6F"  # chess symbols
    "\U0001FA70-\U0001FAFF"  # symbols and pictographs extended
    "\U00002600-\U000026FF"  # miscellaneous symbols
    "\U00002700-\U000027BF"  # dingbats
    "]+"
)

class EmojiIndexer:
    def __init__(self, root_path):
        self.root_path = Path(root_path)
        self.emoji_data = defaultdict(list)
        self.file_stats = defaultdict(int)
        self.emoji_counter = Counter()
        self.context_data = defaultdict(list)
        
    def should_skip_file(self, file_path):
        """Skip binary files, build artifacts, and other non-source files"""
        skip_patterns = [
            '.git/', 'target/', 'node_modules/', '.vscode/',
            '.png', '.jpg', '.jpeg', '.gif', '.ico', '.svg',
            '.exe', '.dll', '.so', '.dylib', '.a', '.lib',
            '.zip', '.tar', '.gz', '.bz2', '.xz',
            'Cargo.lock', '.DS_Store'
        ]
        
        file_str = str(file_path)
        return any(pattern in file_str for pattern in skip_patterns)
    
    def extract_emojis_from_file(self, file_path):
        """Extract all emojis from a single file with context"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                
            lines = content.split('\n')
            file_emojis = []
            
            for line_num, line in enumerate(lines, 1):
                emoji_matches = EMOJI_PATTERN.findall(line)
                if emoji_matches:
                    for emoji_group in emoji_matches:
                        for emoji in emoji_group:
                            self.emoji_counter[emoji] += 1
                            emoji_data = {
                                'emoji': emoji,
                                'file': str(file_path.relative_to(self.root_path)),
                                'line_number': line_num,
                                'context': line.strip(),
                                'file_type': file_path.suffix
                            }
                            file_emojis.append(emoji_data)
                            self.emoji_data[emoji].append(emoji_data)
                            
            if file_emojis:
                self.file_stats[str(file_path.relative_to(self.root_path))] = len(file_emojis)
                
            return file_emojis
            
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
            return []
    
    def scan_directory(self):
        """Recursively scan all files in the directory"""
        print(f"🔍 Scanning {self.root_path} for emojis...")
        
        total_files = 0
        files_with_emojis = 0
        
        for file_path in self.root_path.rglob('*'):
            if file_path.is_file() and not self.should_skip_file(file_path):
                total_files += 1
                emojis = self.extract_emojis_from_file(file_path)
                if emojis:
                    files_with_emojis += 1
                    
        print(f"📊 Scanned {total_files} files, found emojis in {files_with_emojis} files")
        return total_files, files_with_emojis
    
    def categorize_emojis(self):
        """Categorize emojis by type and meaning"""
        categories = {
            'computational': ['🧮', '🔢', '📊', '💻', '⚡', '🔧', '⚙️', '🛠️'],
            'elemental': ['✨', '💫', '🔥', '🌊', '🌪️', '⛈️', '🌈', '☀️'],
            'cosmic': ['🌌', '🚀', '🪐', '⭐', '🌟', '🌠', '🌙', '☄️'],
            'communication': ['📱', '📡', '📢', '📣', '💬', '💭', '🗨️', '🗯️'],
            'crystalline': ['💎', '🔮', '💠', '🔷', '🔶', '🟢', '🟡', '🔴'],
            'void_space': ['🕳️', '⚫', '🌑', '🖤', '◼️', '⬛', '🔳', '🔲'],
            'targeting': ['🎯', '🏹', '🎪', '🎨', '🎭', '🎪', '🎨', '🎯'],
            'nature': ['🌱', '🌿', '🍃', '🌳', '🌲', '🌴', '🌵', '🌾'],
            'animals': ['🐱', '🐶', '🐺', '🦊', '🐸', '🐢', '🦋', '🐝'],
            'faces': ['😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣'],
            'hands': ['👋', '🤚', '🖐️', '✋', '🖖', '👌', '🤌', '🤏'],
            'symbols': ['✅', '❌', '⚠️', '🚫', '🔒', '🔓', '🔑', '🗝️']
        }
        
        categorized = defaultdict(list)
        uncategorized = []
        
        for emoji in self.emoji_counter.keys():
            found_category = False
            for category, emoji_list in categories.items():
                if emoji in emoji_list:
                    categorized[category].append(emoji)
                    found_category = True
                    break
            if not found_category:
                uncategorized.append(emoji)
                
        return dict(categorized), uncategorized
    
    def generate_report(self):
        """Generate comprehensive emoji analysis report"""
        categorized, uncategorized = self.categorize_emojis()
        
        report = []
        report.append("═" * 80)
        report.append("                    🌌 RAGIT EMOJI ANALYSIS REPORT 🌌")
        report.append("                 Comprehensive Emoji Usage Analysis")
        report.append("═" * 80)
        report.append("")
        
        # Executive Summary
        report.append("📋 EXECUTIVE SUMMARY")
        report.append("─" * 20)
        report.append(f"Total Unique Emojis: {len(self.emoji_counter)}")
        report.append(f"Total Emoji Occurrences: {sum(self.emoji_counter.values())}")
        report.append(f"Files with Emojis: {len(self.file_stats)}")
        report.append(f"Average Emojis per File: {sum(self.file_stats.values()) / len(self.file_stats) if self.file_stats else 0:.2f}")
        report.append("")
        
        # Top 20 Most Used Emojis
        report.append("🏆 TOP 20 MOST USED EMOJIS")
        report.append("─" * 28)
        for emoji, count in self.emoji_counter.most_common(20):
            report.append(f"{emoji} : {count} occurrences")
        report.append("")
        
        # Category Analysis
        report.append("🏷️ EMOJI CATEGORIZATION")
        report.append("─" * 23)
        for category, emojis in categorized.items():
            if emojis:
                total_count = sum(self.emoji_counter[emoji] for emoji in emojis)
                report.append(f"{category.upper()}: {len(emojis)} unique, {total_count} total")
                report.append(f"  Emojis: {' '.join(emojis)}")
                report.append("")
        
        if uncategorized:
            total_uncat = sum(self.emoji_counter[emoji] for emoji in uncategorized)
            report.append(f"UNCATEGORIZED: {len(uncategorized)} unique, {total_uncat} total")
            report.append(f"  Emojis: {' '.join(uncategorized)}")
            report.append("")
        
        # File Analysis
        report.append("📁 FILES WITH MOST EMOJIS")
        report.append("─" * 25)
        sorted_files = sorted(self.file_stats.items(), key=lambda x: x[1], reverse=True)
        for file_path, count in sorted_files[:15]:
            report.append(f"{file_path}: {count} emojis")
        report.append("")
        
        # File Type Analysis
        file_type_stats = defaultdict(int)
        for file_path in self.file_stats.keys():
            ext = Path(file_path).suffix or 'no_extension'
            file_type_stats[ext] += self.file_stats[file_path]
            
        report.append("📄 EMOJI USAGE BY FILE TYPE")
        report.append("─" * 27)
        for ext, count in sorted(file_type_stats.items(), key=lambda x: x[1], reverse=True):
            report.append(f"{ext}: {count} emojis")
        report.append("")
        
        # Context Examples
        report.append("🔍 EMOJI CONTEXT EXAMPLES")
        report.append("─" * 25)
        for emoji, count in self.emoji_counter.most_common(10):
            if self.emoji_data[emoji]:
                example = self.emoji_data[emoji][0]
                report.append(f"{emoji} in {example['file']}:{example['line_number']}")
                report.append(f"  Context: {example['context'][:100]}...")
                report.append("")
        
        # Computational Philosophy Analysis
        report.append("🧠 COMPUTATIONAL PHILOSOPHY INSIGHTS")
        report.append("─" * 35)
        
        # Check for our universe system emojis
        universe_emojis = ['🧮', '🔢', '✨', '💫', '🔥', '🌊', '📊', '🎯', '💎', '🕳️', '📱', '🌙', '⭐', '🌌', '🚀', '🪐']
        found_universe = [e for e in universe_emojis if e in self.emoji_counter]
        
        if found_universe:
            report.append(f"🌌 UNIVERSE SYSTEM EMOJIS DETECTED: {len(found_universe)}/16")
            report.append(f"   Found: {' '.join(found_universe)}")
            report.append("   Status: Matrix-to-emoji transformation system ACTIVE ✅")
        else:
            report.append("🌌 UNIVERSE SYSTEM EMOJIS: Not detected in codebase")
            
        report.append("")
        report.append("• Emojis serve as both symbolic representation and functional markers")
        report.append("• High emoji density indicates expressive, human-friendly codebase")
        report.append("• Emoji categorization reveals computational philosophy patterns")
        report.append("• Context analysis shows semantic meaning in technical documentation")
        report.append("")
        
        report.append("═" * 80)
        report.append("                    Report Generated Successfully")
        report.append("        🚀 The Emojis Are Alive and Computing! ✨")
        report.append("═" * 80)
        
        return "\n".join(report)
    
    def save_detailed_data(self, output_file):
        """Save detailed emoji data to JSON file"""
        detailed_data = {
            'summary': {
                'total_unique_emojis': len(self.emoji_counter),
                'total_occurrences': sum(self.emoji_counter.values()),
                'files_with_emojis': len(self.file_stats)
            },
            'emoji_counts': dict(self.emoji_counter),
            'file_stats': dict(self.file_stats),
            'detailed_occurrences': dict(self.emoji_data)
        }
        
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(detailed_data, f, ensure_ascii=False, indent=2)

def main():
    ragit_path = Path.home() / "2025/08/07/ragit"
    
    if not ragit_path.exists():
        print(f"❌ Error: {ragit_path} does not exist")
        return
    
    indexer = EmojiIndexer(ragit_path)
    indexer.scan_directory()
    
    # Generate and display report
    report = indexer.generate_report()
    print(report)
    
    # Save detailed data
    output_file = ragit_path / "emoji_analysis_data.json"
    indexer.save_detailed_data(output_file)
    print(f"\n💾 Detailed data saved to: {output_file}")
    
    # Save report
    report_file = ragit_path / "EMOJI_ANALYSIS_REPORT.md"
    with open(report_file, 'w', encoding='utf-8') as f:
        f.write(report)
    print(f"📄 Report saved to: {report_file}")

if __name__ == "__main__":
    main()
