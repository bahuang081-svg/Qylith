# Qylith Website Deployment Guide

## Website Files Location
All website files are in: `./区块链项目/Qylith/品牌/website/`

## Directory Structure
```
website/
├── index.html          # Homepage
├── architecture.html   # Technical Architecture
├── tokenomics.html     # Token Economics
├── roadmap.html        # Development Roadmap
├── ecosystem.html      # Ecosystem Overview
├── developers.html     # Developer Resources
├── about.html          # About Us
├── whitepaper.html     # Whitepaper
├── 404.html            # 404 Error Page
├── css/
│   └── style.css       # Main Styles
└── js/
    ├── i18n.js         # Internationalization (EN/ZH)
    ├── particles.js    # Particle Animation
    └── main.js         # Main JavaScript
```

## GitHub Deployment Instructions

### Option 1: Push to gh-pages Branch
```bash
cd ./区块链项目/Qylith/品牌/website
git add -A
git commit -m "Qylith website v1.0"
git push origin gh-pages
```

### Option 2: Create New Branch and Deploy
```bash
cd ./区块链项目/Qylith/品牌/website
git branch -M main
git push -u origin main
# Then enable GitHub Pages in repository Settings -> Pages -> Source: main branch
```

### Option 3: Use GitHub CLI
```bash
cd ./区块链项目/Qylith/品牌/website
gh repo set-default bahuang081-svg/Qylith
git push -u origin HEAD
```

## GitHub Pages Setup (if not already configured)
1. Go to: https://github.com/bahuang081-svg/Qylith/settings/pages
2. Source: Deploy from a branch
3. Branch: gh-pages (or main)
4. Save

## Domain Configuration
The domain qylith.xyz should already be configured to point to GitHub Pages.
If not, add a CNAME file with content: `qylith.xyz`

## Features Included
- ✅ 9 HTML Pages (Home, Architecture, Tokenomics, Roadmap, Ecosystem, Developers, About, Whitepaper, 404)
- ✅ Fully Responsive Design (Desktop/Tablet/Mobile)
- ✅ English/Chinese Language Toggle
- ✅ Quantum-themed particle background animation
- ✅ Glassmorphism card design
- ✅ Gradient text effects
- ✅ Scroll animations
- ✅ All links point to correct destinations (GitHub, X/Twitter)
- ✅ Zero external CDN dependencies (except Google Fonts)
- ✅ Clean, semantic HTML5
- ✅ CSS variables for easy customization
