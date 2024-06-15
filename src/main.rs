use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Message)]
#[rtype(result = "()")]
struct ClientMessage(String);

struct MyWs {
    id: usize,
    addr: Addr<ChatServer>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr.do_send(Connect {
            addr: ctx.address().clone(),
            id: self.id,
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.addr.do_send(Disconnect { id: self.id });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => self.addr.do_send(ClientMessage(text.to_string())),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

struct ChatServer {
    sessions: HashMap<usize, Addr<MyWs>>,
    counter: Arc<Mutex<usize>>,
}

impl ChatServer {
    fn new() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    fn get_next_id(&self) -> usize {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        *counter
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

struct Connect {
    addr: Addr<MyWs>,
    id: usize,
}

struct Disconnect {
    id: usize,
}

impl Message for Connect {
    type Result = ();
}

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        for session in self.sessions.values() {
            session.do_send(ClientMessage(msg.0.clone()));
        }
    }
}

impl Handler<ClientMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

async fn websocket_handler(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<ChatServer>>) -> Result<HttpResponse, Error> {
    let addr = srv.get_ref().clone();
    let id = addr.send(GetNextId).await.unwrap();
    ws::start(MyWs { id, addr }, &req, stream)
}

#[derive(Message)]
#[rtype(result = "usize")]
struct GetNextId;

impl Handler<GetNextId> for ChatServer {
    type Result = usize;

    fn handle(&mut self, _: GetNextId, _: &mut Self::Context) -> Self::Result {
        self.get_next_id()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_server.clone()))
            .route("/ws/", web::get().to(websocket_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
