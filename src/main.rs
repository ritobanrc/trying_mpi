use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    //println!("Printing from process {} of {} at time {:?}", rank, size, std::time::Instant::now());

    //match rank {
    //    0 => {
    //        let msg = vec![4.0f64, 8.0, 15.0];
    //        world.process_at_rank(rank + 1).send(&msg[..]);
    //    }
    //    1 => {
    //        let (msg, status) = world.any_process().receive_vec::<f64>();
    //        println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status);
    //    }
    //    _ => unreachable!()
    //}
}
