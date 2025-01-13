// components/ParticlesBackground.tsx
'use client';

import { useCallback } from "react";
import Particles from "@tsparticles/react";
import { loadSlim } from "@tsparticles/slim";
import type { Engine } from "@tsparticles/engine";

export default function ParticlesBackground() {
  const particlesInit = useCallback(async (engine: Engine) => {
    await loadSlim(engine);
  }, []);

  return (
    <Particles
      id="tsparticles"
      options={{
        background: {
          color: "#000000"
        },
        particles: {
          color: {
            value: ["#aa73ff", "#f8c210", "#83d238", "#33b1f8"]
          },
          number: {
            value: 50, 
            density: {
              enable: true,
            }
          },
          opacity: {
            value: 0.6
          },
          size: {
            value: 2
          },
          move: {
            enable: true,
            speed: 1 // Reduced speed for better performance
          },
          links: {
            enable: true,
            distance: 120,
            color: "#ffffff",
            opacity: 0.4,
            width: 1
          }
        },
        interactivity: {
          events: {
            onHover: {
              enable: true,
              mode: "grab"
            }
          },
          modes: {
            grab: {
              distance: 140,
              links: {
                opacity: 1
              }
            }
          }
        },
        fps_limit: 60 // Add FPS limit for better performance
      }}
    />
  );
}
