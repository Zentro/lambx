# Latifa (our Unity 3D asset mascot): Alice Messages Bob (and signs using the) XEdDSA (protocol from Signal written in Rust)
Read the specification document here https://signal.org/docs/specifications/xeddsa/

## Inspiration
The PQXDH protocol recently revealed by Signal seems promising for safe and anonymous messaging even with the advent of quantum computing possibly breaking our traditional asymmetric key exchange algorithms. We (ambitiously) wanted to implement this protocol alongside a message app using it as a backbone for secure key exchanges.

## What it does
It does not have PQXDH, but it does have the signature protocol named XEdDSA that is used in PQXDH. It basically allows you to use X25519 keys alongside an authentication protocol that would typically require its own set of keys. Currently, the functionality of this library is limited to just signing and verifying (being that these are the main functions of a signing algorithm).

## How we built it
We first had a group reading session over the paper specifying the details of implementation then read reference material to bridge the gaps in our understanding. Then, we spent some time finding good libraries for operations on unsigned 256-bit integers along with one that could do operations on Curve25519. We then spent a lot of time trying to translate the mathematical operations onto a whiteboard, rationalizing our assumptions and arithmetic. After this, we started putting it down into code, which also necessitated a lot of trial and error in figuring out how scalars and curve points interacted with one another.

## Challenges we ran into
- Not being able to easily test with presupplied vectors of results made it much more difficult to pinpoint where the problem is.
- Working with hash functions meant that small changes completely changed representations of the result. The same happened with operations along the curve as small mistakes quickly catapulted into nonsensical numbers.
- Libraries were somewhat difficult to work with and Rust's compiler errors were sometimes arcane.
- It was excruciatingly difficult to piece together the information presented as they were on the specifications document as many crucial elements were hidden in reference material.
- Setting up the Raspberry Pi was a nightmare, though we did not use it much.

## Accomplishments that we're proud of
- But all in all, we're proud of presenting possibly the first proof-of-concept of the XEdDSA algorithm in Rust that will be publicly available. Though we do not guarantee full cryptographic security (given the peculiarities of cryptography), it presents the source material faithfully enough that we're confident to say that it could be foundational for a more fleshed out implementation.

## What we learned
ECC. The biggest part was simply understanding how finite fields and the numbers within them worked, alongside with how it is used in creating algorithms much faster than RSA's conventional Diffie-Hellman.

## What's next for L.A.M.B.X
We are planning for our XEdDSA implementation to be the foundational signature verification scheme for the PQXDH protocol we plan to build for our next Hackathon. It will then also be a part of a messaging app focused on anonymous communication through a small server.
