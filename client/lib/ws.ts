import WebSocket from 'isomorphic-ws';

let socket: WebSocket | null = null;

export const connect = (url: string) => {
  socket = new WebSocket(url);

  socket.onopen = () => {
    console.log('WebSocket connected');
  };

  socket.onmessage = (message: { data: any; }) => {
    console.log('Received:', message.data);
  };

  socket.onclose = () => {
    console.log('WebSocket disconnected');
  };

  socket.onerror = (error: any) => {
    console.error('WebSocket error:', error);
  };

    return socket;
};

export const sendMessage = (message: string) => {
  if (socket && socket.readyState === WebSocket.OPEN) {
    socket.send(message);
  } else {
    console.error('WebSocket is not connected');
  }
};
