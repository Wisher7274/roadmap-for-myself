<template>
  <canvas ref="canvas" class="fixed inset-0 pointer-events-none z-0"></canvas>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';

const canvas = ref(null);
let ctx, particles = [], animationId;

class Particle {
  constructor(w, h) {
    this.x = Math.random() * w;
    this.y = Math.random() * h;
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = (Math.random() - 0.5) * 0.5;
    this.size = Math.random() * 2 + 1;
    this.opacity = Math.random() * 0.5 + 0.2;
  }
  update(w, h) {
    this.x += this.vx;
    this.y += this.vy;
    if (this.x < 0 || this.x > w) this.vx *= -1;
    if (this.y < 0 || this.y > h) this.vy *= -1;
  }
  draw(ctx) {
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(255, 255, 255, ${this.opacity})`;
    ctx.fill();
  }
}

function init() {
  const c = canvas.value;
  if (!c) return;
  ctx = c.getContext('2d');
  
  const resize = () => {
    c.width = window.innerWidth;
    c.height = window.innerHeight;
  };
  resize();
  window.addEventListener('resize', resize);

  const count = Math.floor((c.width * c.height) / 15000);
  particles = Array.from({ length: count }, () => new Particle(c.width, c.height));

  function animate() {
    ctx.clearRect(0, 0, c.width, c.height);
    particles.forEach(p => {
      p.update(c.width, c.height);
      p.draw(ctx);
    });
    // 连线
    for (let i = 0; i < particles.length; i++) {
      for (let j = i + 1; j < particles.length; j++) {
        const dx = particles[i].x - particles[j].x;
        const dy = particles[i].y - particles[j].y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist < 100) {
          ctx.beginPath();
          ctx.strokeStyle = `rgba(255,255,255,${0.1 * (1 - dist/100)})`;
          ctx.lineWidth = 1;
          ctx.moveTo(particles[i].x, particles[i].y);
          ctx.lineTo(particles[j].x, particles[j].y);
          ctx.stroke();
        }
      }
    }
    animationId = requestAnimationFrame(animate);
  }
  animate();

  return () => {
    window.removeEventListener('resize', resize);
    cancelAnimationFrame(animationId);
  };
}

onMounted(() => {
  const cleanup = init();
  onUnmounted(cleanup);
});
</script>