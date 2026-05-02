/**
 * Qylith - Particles Background
 * Quantum-Secured AI-Native Layer 1
 */

class ParticleNetwork {
  constructor(canvasId, options = {}) {
    this.canvas = document.getElementById(canvasId);
    if (!this.canvas) return;
    
    this.ctx = this.canvas.getContext('2d');
    this.particles = [];
    this.animationId = null;
    
    this.options = {
      particleCount: options.particleCount || 80,
      particleColor: options.particleColor || '#00D4FF',
      lineColor: options.lineColor || 'rgba(0, 212, 255, 0.15)',
      particleSize: options.particleSize || 2,
      speed: options.speed || 0.5,
      maxDistance: options.maxDistance || 150,
      ...options
    };
    
    this.init();
    this.animate();
    this.handleResize();
  }
  
  init() {
    this.resize();
    this.createParticles();
  }
  
  resize() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }
  
  createParticles() {
    this.particles = [];
    const count = Math.min(this.options.particleCount, Math.floor((this.canvas.width * this.canvas.height) / 15000));
    
    for (let i = 0; i < count; i++) {
      this.particles.push({
        x: Math.random() * this.canvas.width,
        y: Math.random() * this.canvas.height,
        vx: (Math.random() - 0.5) * this.options.speed,
        vy: (Math.random() - 0.5) * this.options.speed,
        size: Math.random() * this.options.particleSize + 1,
        opacity: Math.random() * 0.5 + 0.3
      });
    }
  }
  
  animate() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    
    // Update and draw particles
    this.particles.forEach((p, i) => {
      // Update position
      p.x += p.vx;
      p.y += p.vy;
      
      // Boundary check
      if (p.x < 0 || p.x > this.canvas.width) p.vx *= -1;
      if (p.y < 0 || p.y > this.canvas.height) p.vy *= -1;
      
      // Draw particle
      this.ctx.beginPath();
      this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
      this.ctx.fillStyle = this.options.particleColor;
      this.ctx.globalAlpha = p.opacity;
      this.ctx.fill();
      
      // Draw connections
      this.particles.slice(i + 1).forEach(p2 => {
        const dx = p.x - p2.x;
        const dy = p.y - p2.y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        
        if (dist < this.options.maxDistance) {
          const opacity = (1 - dist / this.options.maxDistance) * 0.3;
          this.ctx.beginPath();
          this.ctx.moveTo(p.x, p.y);
          this.ctx.lineTo(p2.x, p2.y);
          this.ctx.strokeStyle = this.options.lineColor.replace('0.15', opacity.toFixed(2));
          this.ctx.lineWidth = 0.5;
          this.ctx.stroke();
        }
      });
    });
    
    this.ctx.globalAlpha = 1;
    this.animationId = requestAnimationFrame(() => this.animate());
  }
  
  handleResize() {
    window.addEventListener('resize', () => {
      this.resize();
      this.createParticles();
    });
  }
  
  destroy() {
    if (this.animationId) {
      cancelAnimationFrame(this.animationId);
    }
  }
}

// Initialize particles on page load
document.addEventListener('DOMContentLoaded', () => {
  // Create particle network with quantum theme
  new ParticleNetwork('particles', {
    particleCount: 100,
    particleColor: '#00D4FF',
    lineColor: 'rgba(0, 212, 255, 0.15)',
    particleSize: 2,
    speed: 0.4,
    maxDistance: 180
  });
});

// Export for global access
window.ParticleNetwork = ParticleNetwork;
