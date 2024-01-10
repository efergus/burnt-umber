import * as THREE from 'three';

const stick_thetas = [
    0,
    Math.PI / 2,
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
            let cross: number | undefined = undefined;
            for (const stick of stick_thetas) {
                if (this.theta > stick !== new_theta > stick) {
                    cross = stick;
                    break;
                }
            }

            if (cross !== undefined || this.stick !== 0) {
                this.stick += delta.y - this.stick * deltaT;
                this.theta = cross ?? this.theta;
            } else {
                this.theta = new_theta;
            }
            if (Math.abs(this.stick) >= 60) {
                this.stick = 0;
                this.theta = new_theta;
            }
            this.phi += delta.x * 0.01;
            this.radius += delta.z * 0.04;

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
