//fn main() {
    //println!("Hello, world!");
//}

//extern crate pcsc;

//use pcsc::*;

//fn main() {
    //// Get a context.
    //let ctx = Context::establish(Scope::User).expect("failed to establish context");

    //// First allocate the required buffer.
    //let len = ctx.list_readers_len().expect("failed to list readers needed len");
    //let mut readers_buf = vec![0; len];
    //// Alternatively, we could have just used a sufficiently large
    //// statically sized, stack allocated buffer instead, like we do in
    //// other examples:
    //// let mut readers_buf = [0; 2048];

    //let names = ctx.list_readers(&mut readers_buf).expect("failed to list readers");
    //for name in names {
        //println!("{:?}", name);
    //}
//}


// Example of communication with a smart card.

extern crate pcsc;

use pcsc::*;


// Example of how to monitor card & card reader state changes.

//extern crate pcsc;

//use pcsc::*;

fn main() {
    let ctx = Context::establish(Scope::User).expect("failed to establish context");

    let mut readers_buf = [0; 2048];
    let mut reader_states = vec![
        // Listen for reader insertions/removals.
        ReaderState::new(PNP_NOTIFICATION(), STATE_UNAWARE),
    ];
    loop {
        // Remove dead readers.
        fn is_dead(rs: &ReaderState) -> bool {
            rs.event_state().intersects(STATE_UNKNOWN | STATE_IGNORE)
        }
        for rs in &reader_states {
            if is_dead(rs) {
                println!("Removing {:?}", rs.name());
            }
        }
        reader_states.retain(|rs| !is_dead(rs));

        // Add new readers.
        let names = ctx.list_readers(&mut readers_buf).expect("failed to list readers");
        for name in names {
            if !reader_states[1..].iter().any(|rs| rs.name() == name) {
                println!("Adding {:?}", name);
                reader_states.push(ReaderState::new(name, STATE_UNAWARE));
            }
        }

        // Update the view of the state to wait on.
        for rs in &mut reader_states {
            rs.sync_current_state();
        }

        // Wait until the state changes.
        ctx.get_status_change(None, &mut reader_states).expect("failed to get status change");

        // Print current state.
        println!();
        for rs in &reader_states[1..] {
            println!("{:?} {:?}", rs.name(), rs.event_state());
            if rs.event_state().intersects(pcsc::STATE_PRESENT) {
                print_uid(&ctx);
            }
        }
        //printUID(&ctx);
    }
}

fn print_uid(ctx: &pcsc::Context) {
    // Get a context.
    //let ctx = Context::establish(Scope::User).expect("failed to establish context");

    // List connected readers.
    let mut readers_buf = [0; 2048];
    let readers = ctx.list_readers(&mut readers_buf).expect("failed to list readers").collect::<Vec<_>>();
    println!("Readers: {:?}", readers);

    if readers.is_empty() {
        return;
    }

    {
        // Try to connect to a card in the first reader.
        let mut card = ctx.connect(readers[0], ShareMode::Exclusive, PROTOCOL_ANY).expect("failed to connect to card");

        {
            // Start an exclusive transaction (not required -- can work on card directly).
            let tx = card.transaction().expect("failed to begin card transaction");

            let apdu2 = b"\xff\xca\x00\x00\x00";
            let mut rapdu_buf2 = [0; MAX_BUFFER_SIZE];
            let rapdu2 = tx.transmit(apdu2, &mut rapdu_buf2).expect("failed to transmit APDU to card");
            println!("MYVAL: {:?}", rapdu2);
			println!("size: {:?}", rapdu2.len());
			
			
			let aambal: [u8; 6] = [164u8, 158u8, 218u8, 1u8, 144u8, 0u8];
			let bdekens: [u8; 6] = [164u8, 170u8, 47u8, 1u8, 144u8, 0u8];
			let nsavois: [u8; 6] = [212u8, 86u8, 68u8, 1u8, 144u8, 0u8];

			let mut card_holder = "";
			if rapdu2 == aambal {
				card_holder = "aambal";
			}
			if rapdu2 == bdekens {
				card_holder = "bdekens";
			}
			if rapdu2 == nsavois {
				card_holder = "nsavois";
			}
			println!("{:?}",card_holder);


			//let uid: &str = std::str::from_utf8(rapdu2).unwrap();

            tx.end(Disposition::LeaveCard).map_err(|(_, err)| err).expect("failed to end transaction");
        }

        // Can either disconnect explicity, which allows error handling,
        // and setting the disposition method, or leave it to drop, which
        // swallows any error and hardcodes ResetCard.
        //card.disconnect(Disposition::ResetCard).map_err(|(_, err)| err).expect("failed to disconnect from card");
    }

    // Can either release explicity, which allows error handling,
    // or leave it to drop, which swallows any error.
    //ctx.release().map_err(|(_, err)| err).expect("failed to release context");
}

