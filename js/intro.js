let framesSinceInit;
let introID;
let introElem;
let name = "Jack Dinsmore";
let millisecondsBetweenFramesIntro = 80;

function introFrame () {
    if (framesSinceInit < 2 * name.length) {
        if (framesSinceInit % 2 == 0){
            let index = Math.floor(framesSinceInit / 2);
            introElem.innerHTML += name.charAt(index);
        }
    }
    if (framesSinceInit == 36) {
        introElem.innerHTML += " >";
        document.getElementById("intronav").style.visibility = "visible";
        clearInterval(introID);
    }
    framesSinceInit++;
}

function loadIntro() {
    framesSinceInit = 0;
    introElem = document.getElementById("intro");
    introID = setInterval(introFrame, millisecondsBetweenFramesIntro);
}
