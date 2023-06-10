let playerField = document.querySelector("#player");
let board = document.querySelectorAll(".cell");

let statusDiv = document.getElementById('status');
let roundDiv = document.getElementById('round');
let re = document.getElementById('re');

let hisTemplate = document.getElementById('his');
let hisList = document.getElementById('his-list');

var STATE = {
    connected: false,
}

function addHis(text) {
    var node = hisTemplate.content.cloneNode(true);
    var hisDiv = node.querySelector(".hisDiv");
    hisDiv.innerText = text;
    hisList.appendChild(node);
}
function subscribe(uri) {
    var retryTime = 1;

    function connect(uri) {
        const events = new EventSource(uri);

        events.addEventListener("message", (ev) => {
            console.log("raw data", JSON.stringify(ev.data));
            console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
            addHis(JSON.stringify(JSON.parse(ev.data)));
            const msg = JSON.parse(ev.data);
            if (!"BX" in msg || !"BO" in msg || !"prime" in msg) return;
            setChar(msg.BX<0?0:msg.BX,msg.BO<0?0:msg.BO)
            if (msg.BO<0){
                roundDiv.innerText = "won : X";
            }else if (msg.BX<0){
                roundDiv.innerText = "won : O";
            }else{
                roundDiv.innerText = "round : " + (msg.prime ? "X" : "O");
            }

        });

        events.addEventListener("open", () => {
            setConnectedStatus(true);
            console.log(`connected to event stream at ${uri}`);
            retryTime = 1;
            sent(-1);
        });

        events.addEventListener("error", () => {
            setConnectedStatus(false);
            events.close();

            let timeout = retryTime;
            retryTime = Math.min(64, retryTime * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => connect(uri), (() => timeout * 1000)());
        });
    }

    connect(uri);
}

function setConnectedStatus(status) {
    STATE.connected = status;
    statusDiv.className = (status) ? "connected" : "reconnecting";
}


function setChar(numX,numO) {

    for (let i = 0; i < 9; i++) {
        board.item(i).innerText = "";

        if((numX & Math.pow(2,i)) === Math.pow(2,i)){
            board.item(i).innerText = "X";
        }
        if((numO & Math.pow(2,i)) === Math.pow(2,i)){
            board.item(i).innerText = "O";
        }
    }
}


function sAdd(res,num) {

    return ((res &  Math.pow(2,num)) !== Math.pow(2,num)) ? res + Math.pow(2,num) : res;

}

function sent(num) {
    const player = playerField.value === "x";
    // if (!player || !num) return;

    if (STATE.connected) {
        fetch("/message", {
            method: "POST",
            body: new URLSearchParams({ player , num }),
        }).then((response) => {
            if (response.ok) {

            }
        });
    }
}

function init() {

    for (let i = 0; i < board.length; i++) {
        board.item(i).addEventListener("click",()=>{

            sent(i);

        })
    }
    re.addEventListener("click",()=>{

        sent(-1);

    })

    subscribe("/events");
}

init();

