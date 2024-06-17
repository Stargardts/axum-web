document.addEventListener("DOMContentLoaded", function () {
  var deviceWidth = window.innerWidth;
  console.log("Width of the device is: " + deviceWidth);
  // Send width of the device to the server
  fetch("/width", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ width: deviceWidth }),
  })
    .then((res) => res.json())
    .then((data) => console.log(data));
  var contentHeight = document.body.scrollHeight;
  var viewportHeight = window.innerHeight;
  // Try to find footer element and log Found otherwise log console not found
  var footer = document.getElementById("footer");

  if (contentHeight > viewportHeight) {
    footer.classList.add("footer-relative");
    console.log("Footer is relative");
  } else {
    footer.classList.add("footer-fixed");
    console.log("Footer is fixed");
  }

  // If the Document contains both "TextArea and Submit-Button" then change the width of the textArea
  var textArea = document.getElementById("userInput");
  var submitButton = document.getElementById("submit-button");
  if (textArea && submitButton) {
    console.log("TextArea and Submit-Button are found");
    if (deviceWidth > 700) {
      textArea.classList.add("mx-20");
      submitButton.classList.add("mr-20");
    } else {
      textArea.classList.add("mx-0");
      submitButton.classList.add("mr-0");
    }
  } else {
  }

  submitButton.addEventListener("click", sendData);
});

function sendData() {
  const userInput = document.getElementById("userInput").value;
  fetch("/userInput", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ input: userInput }),
  })
    .then((response) => response.json())
    .then((data) => {
      console.log("Success:", data);
      // replace the inner html div with id "chat" with the response from the server as paragraph
      var response = data.response;
      var key = data.key;
      console.log("Response from the server is: " + response);
      var chat = document.getElementById("chat");
      chat.classList.add("mx-15");
      chat.innerHTML = `<p>${response}</p>`;
      chat.appendChild(document.createElement("br"));
      var hidden = chat.appendChild(document.createElement("div"));
      var span = hidden.appendChild(document.createElement("span"));
      span.style.display = "none";
      span.classList.add("text-sm");
      span.innerHTML = key;
      button = hidden.appendChild(document.createElement("button")).innerHTML = "Show key";
      button.addEventListener("click", function () {
        span.style.display = "block";
      });
    })
    .catch((error) => {
      console.error("Error:", error);
    });
}
