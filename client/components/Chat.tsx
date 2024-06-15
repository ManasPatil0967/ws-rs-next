import React, { useEffect, useState } from 'react';
import { connect, sendMessage } from '../lib/ws';

const Chat: React.FC = () => {
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<string[]>([]);

  useEffect(() => {
    const socket = connect('ws://localhost:8080/ws/');

    const onMessage = (event: MessageEvent) => {
      setMessages(prevMessages => [...prevMessages, event.data]);
    };

    if (socket) {
      socket.onmessage = onMessage;
    }

    return () => {
      if (socket) {
        socket.onmessage = null;
      }
    };
  }, []);

  const handleSend = () => {
    sendMessage(message);
    setMessage('');
  };

  return (
    <div>
      <div>
        {messages.map((msg, index) => (
          <div key={index}>{msg}</div>
        ))}
      </div>
      <input
        type="text"
        value={message}
        onChange={e => setMessage(e.target.value)}
        className='text-black'
      />
      <button onClick={handleSend}>Send</button>
    </div>
  );
};

export default Chat;
