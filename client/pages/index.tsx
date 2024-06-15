import Head from 'next/head';
import Chat from '../components/Chat';

const Home: React.FC = () => {
  return (
    <div>
      <Head>
        <title>Chat App</title>
      </Head>
      <Chat />
    </div>
  );
};

export default Home;