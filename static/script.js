document.getElementById("sendButton").onclick = function (){
    console.log("fn called")
    // send_message()
    // get_all_messages()
};

async function send_message(){
    // first send the msg  to the chatBox
    
    // then send that chat to server
    const response = await fetch('http://127.0.0.1:8080/get_messages');
    const myJson = await response.json();
    console.log(myJson)
}

function get_all_messages(){
    const request = new XMLHttpRequest();
    request.open("GET","http://127.0.0.1:8080/get_messages");
}