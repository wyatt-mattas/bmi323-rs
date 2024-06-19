#[derive(Debug)]
pub enum Error<E> {
    Comm(E),
    Other,
}
