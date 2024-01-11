import * as THREE from 'three';

type CameraStop = {
    angle: number,
    threshold?: number,
}

const stick_thetas: CameraStop[] = [
    {
        angle: 0,
    },
    {
        angle: Math.PI / 4,
        threshold: 20,
    },
    {
        angle: Math.PI / 2,
    }
]

function spherical_to_cartesian(theta: number, phi: number, radius: number) {
    return new THREE.Vector3(
        radius * Math.cos(theta) * Math.cos(phi),
        radius * Math.sin(theta),
        radius * Math.cos(theta) * Math.sin(phi)
    );
}

export interface CameraController {
    stick: number;
    theta: number;
    phi: number;
    radius: number;
    lookAt: THREE.Vector3;

    on_move(delta: THREE.Vector3): void;
    update(): void;
}

export function cameraController(camera: THREE.PerspectiveCamera): CameraController {
    const controller = {
        stick: 1,
        theta: Math.PI / 2,
        phi: 0,
        radius: 3,
        lookAt: new THREE.Vector3(0, 1, 0),
        latest: Date.now(),

        on_move(delta: THREE.Vector3) {
            const now = Date.now();
            const deltaT = (now - this.latest) / 1000;
            this.latest = now;
            const new_theta = Math.max(
                Math.min(this.theta + delta.y * 0.01, Math.PI * 1.5),
                -Math.PI / 2
            );
            let cross: CameraStop | undefined = undefined;
            for (const stick of stick_thetas) {
                const angle = stick.angle;
                if (this.theta > angle !== new_theta > angle || this.theta === angle) {
                    cross = stick;
                    break;
                }
            }

            const vert = Math.max(Math.abs(delta.y) - Math.abs(delta.x), 0)
            if (cross !== undefined) {
                this.stick += vert - this.stick * deltaT;
                this.theta = cross?.angle ?? this.theta;
            } else {
                this.theta = new_theta;
            }
            if (Math.abs(this.stick) >= (cross?.threshold ?? 40)) {
                this.stick = 0;
                this.theta = new_theta;
            }
            this.phi += delta.x * 0.01;
            this.radius += delta.z * 0.04;
            if (this.phi > Math.PI) {
                this.phi -= Math.PI * 2;
            }
            else if (this.phi < -Math.PI) {
                this.phi += Math.PI * 2;
            }

            this.radius = Math.max(this.radius, 0.1);
        },

        update() {
            const radius = this.radius + Math.cos(this.theta) ** 2 * 0.5;

            const position = spherical_to_cartesian(this.theta, this.phi, radius);
            const up = spherical_to_cartesian(this.theta + Math.PI / 2, this.phi, 1);
            up.normalize();
            this.lookAt.y = (Math.sin(this.theta) + 1) / 2;
            position.add(this.lookAt);
            camera.up.copy(up);
            camera.position.copy(position);
            camera.lookAt(this.lookAt);
        }
    };
    controller.on_move(new THREE.Vector3(0, 0, 0));
    return controller;
}
