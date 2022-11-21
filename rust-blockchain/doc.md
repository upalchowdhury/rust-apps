1. P2P networking, 
2. blockchain data structures, 
3. proof-of-work algorithm,  
4. cryptography, 
5. account management, 
6. wallet, 
7. consensus, 
8. data stores, 
9. privacy features



Find a piece of data — in this case, the nonce and a number, which, together with our block data hashed using SHA256, will give us a hash starting with two zeros. if it would take us 64000 iterations to calculate a fitting hash (starting with two zeros), the nonce would be 63999 (1 less, since it starts at 0).


basic network transport protocols to security layer protocols and multiplexing.

FloodSub protocol, a simple publish/subscribe protocol, for communication between the nodes.

if a request is sent by triggering LocalChainRequest with the peer_id of another node in the system to send us their chain back as response.

AppBehaviour holds
     FloodSub instance for pub/sub communication 
     
     Mdns instance, which will enable us to automatically find other nodes on our local network (but not outside of it).


To handle 
    - incoming messages, 
    - lazy initialization, and  
    - keyboard-input by the client’s user, 
     we define the EventType enum, which will help us send events across the application to keep our application state in sync with incoming and outgoing network traffic.


We check wether we’re actually the receiver of said piece of data and, if so, log the incoming blockchain and attempt to execute our consensus. If it’s valid and longer than our chain, we replace our chain with it. Otherwise, we keep our own chain.




If the incoming data is a LocalChainRequest, we check whether we’re the ones they want the chain from, checking the from_peer_id. If so, we simply send them a JSON version of our local blockchain. The actual sending part is in another part of the code, but for now, we simply send it through our event channel for responses.

Finally, if it’s a Block that’s incoming, that means someone else mined a block and wants us to add it to our local chain. We check whether the block is valid and, if it is, add it.



