import * as THREE from 'three';

const unit = {
    x: new THREE.Vector3(1, 0, 0),
    y: new THREE.Vector3(0, 1, 0),
    z: new THREE.Vector3(0, 0, 1)
};

function spherical_to_cartesian(theta: number, phi: number, radius: number) {
    // const sign = Math.sign(Math.cos(theta));
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
    // up: THREE.Vector3,
    lookAt: THREE.Vector3;

    on_move(delta: THREE.Vector3): void;
}

export function cameraController(camera: THREE.PerspectiveCamera): CameraController {
    const controller = {
        stick: 1,
        theta: Math.PI / 2,
        phi: 0,
        radius: 1.5,
        lookAt: new THREE.Vector3(0, 1, 0),

        on_move(delta: THREE.Vector3) {
            const new_theta = Math.max(
                Math.min(this.theta + delta.y * 0.01, Math.PI * 1.5),
                -Math.PI / 2
            );
            const cross =
                this.theta > Math.PI / 2 !== new_theta > Math.PI / 2 && this.theta !== Math.PI / 2;
            // console.log(cross, this.stick, this.theta, new_theta);

            if (cross) {
                this.stick += delta.y;
                this.theta = Math.PI / 2;
            } else if (this.stick !== 0) {
                this.stick += delta.y;
            } else {
                this.theta = new_theta;
            }
            if (Math.abs(this.stick) >= 40) {
                this.stick = 0;
                this.theta = new_theta;
            }
            this.phi += delta.x * 0.01;
            this.radius += delta.z * 0.04;

            this.radius = Math.max(this.radius, 0.1);
            const radius = this.radius + Math.cos(this.theta) ** 2 * 0.5;

            const position = spherical_to_cartesian(this.theta, this.phi, radius);
            const up = spherical_to_cartesian(this.theta + Math.PI / 2, this.phi, 1);
            up.normalize();
            this.lookAt.y = (Math.sin(this.theta) + 1) / 2;
            position.add(this.lookAt);
            camera.up.copy(up);
            camera.position.copy(position);
            camera.lookAt(this.lookAt);

            // console.log("MOVED", this.phi, ...camera.position)
        }
    };
    controller.on_move(new THREE.Vector3(0, 0, 0));
    return controller;
}
