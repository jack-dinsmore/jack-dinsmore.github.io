const density = 0.0001;
let container;
let stars = [];
let repeat_id;
const millisecondsBetweenFrames = 100;
let star_velocity = 0.1;

class BadRNG {
    constructor(seed) {
        self.num = seed
    }

    gen() {
        self.num = (self.num * 9301 + 49297) % 233280;
        return self.num / 233280;
    }
}

let rng = new BadRNG(9573021);
let now = Date.now() / millisecondsBetweenFrames;

class Star {
    constructor() {
        let theta = rng.gen() * 2 * Math.PI;
        this.vx = star_velocity * Math.cos(theta);
        this.vy = star_velocity * Math.sin(theta);
        this.x = Math.trunc(rng.gen() * screen.width + this.vx * now) % screen.width;
        this.y = Math.trunc(rng.gen() * screen.height + this.vy * now) % screen.height;

        this.elem = document.createElement("div");
        this.elem.style.background="rgb("+
            Math.trunc(128 + rng.gen() * 128).toString() + ", " +
            Math.trunc(128 + rng.gen() * 128).toString() + ", " +
            Math.trunc(128 + rng.gen() * 128).toString() + ")";
        this.elem.id="animate";
        this.elem.style.top = Math.trunc(this.y) + "px";
        this.elem.style.left = Math.trunc(this.x) + "px";
        container.append(this.elem);
    }

    move() {
        this.x += this.vx;
        this.y += this.vy;
        if (this.x < 0) { this.x += screen.width; }
        if (this.y < 0) { this.y += screen.height; }
        if (this.x > screen.width) { this.x -= screen.width; }
        if (this.y > screen.height) { this.y -= screen.height; }
        this.elem.style.top = Math.trunc(this.y) + "px";
        this.elem.style.left = Math.trunc(this.x) + "px";
    }
}

function frame() {
    for (let i = 0; i < stars.length; i++) {
        stars[i].move();
    }
}

window.onload = function() {
    container = document.getElementById("background");
    let numParticles = screen.width * screen.height * density;
    for (let i = 0; i < numParticles; i++) {
        stars.push(new Star());
    }

    repeat_id = setInterval(frame, millisecondsBetweenFrames);

    try {
        loadIntro()
    }
    catch (err) {}
}
