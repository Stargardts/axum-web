// Ensure browser compatibility for mediaDevices
navigator.mediaDevices.getUserMedia = navigator.mediaDevices.getUserMedia ||
                                      navigator.mediaDevices.webkitGetUserMedia ||
                                      navigator.mediaDevices.mozGetUserMedia;

let mediaRecorder;
let audioChunks = [];

document.getElementById('recordButton').addEventListener('click', async () => {
  if (mediaRecorder && mediaRecorder.state === 'recording') {
    mediaRecorder.stop();
    return;
  }

  const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
  mediaRecorder = new MediaRecorder(stream);

  mediaRecorder.ondataavailable = (event) => {
    audioChunks.push(event.data);
  };

  mediaRecorder.onstop = () => {
    const audioBlob = new Blob(audioChunks, { type: 'audio/wav' });
    sendAudioToServer(audioBlob);
    audioChunks = [];
  };

  mediaRecorder.start();
});


function sendAudioToServer(audioBlob) {
  const formData = new FormData();
  formData.append('audio', audioBlob, 'recording.wav');

  fetch('http://0.0.0.0:8080/upload', {
    method: 'POST',
    body: formData,
  })
  .then(response => response.json())
  .then(data => {
    console.log('Success:', data);
  })
  .catch((error) => {
    console.error('Error:', error);
  });
}


