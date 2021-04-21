import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as rust_hello_idl, canisterId as rust_hello_id } from 'dfx-generated/rust_hello';

const agent = new HttpAgent();
const rust_hello = Actor.createActor(rust_hello_idl, { agent, canisterId: rust_hello_id });

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  const greeting = await rust_hello.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
