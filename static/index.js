let bits = []
let free_bit = false;

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function get_bits(n) {
    for (let i = 0; i < n; i++) {
        get_bit();
        await sleep(20);
    }
}

function get_bit() {
    let x;
    if (bits.length == 0) {
        if (x != undefined && x.readyState == XMLHttpRequest.OPENED) {
            return;
        }

        x = new XMLHttpRequest();
        x.onreadystatechange = () => {
            if (x.readyState == XMLHttpRequest.DONE && x.status == 200) {
                for (let bit of x.response) {
                    bits.push(bit);
                }

                if (!free_bit) {
                    document.getElementById("free_bit").innerHTML = bits.pop()
                    free_bit = true;
                } else {
                    add_bit(bits.pop())
                }
            }
        };
        x.open('GET', "/bits");
        x.responseType = "text";
        x.setRequestHeader("Content-Type", "application/text");
        x.send();
    } else {
        add_bit(bits.pop())
    }
}

function add_bit(bit) {
    document.getElementById("bits").innerHTML = document.getElementById("bits").innerHTML + " " + bit;
}

function send(bit) {
    x = new XMLHttpRequest();
    x.open('POST', bit ? "/one" : "/zero");
    x.responseType = "text";
    x.send();
}

get_bit();