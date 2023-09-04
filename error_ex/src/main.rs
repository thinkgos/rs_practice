// use std::error::Error;
use thiserror::Error;

// 使用 Debug 宏 实现  fmt::Debug trait
// 使用  thiserror 宏 实现 std::error::Error 和 fmt::Display trait
// impl Error for LockError {}
// impl fmt::Display for LockError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::MechanicalError(code) => write!(f, "Mechanical error: {}", code),
//             Self::NetworkError => write!(f, "Network error"),
//             Self::NotAuthorized => write!(f, "Not authorized"),
//         }
//     }
// }
// thiserror 宏 实现 fmt::Display trait 形式
// #[error("{var}")] ⟶ write!("{}", self.var)
// #[error("{0}")] ⟶ write!("{}", self.0)
// #[error("{var:?}")] ⟶ write!("{:?}", self.var)
// #[error("{0:?}")] ⟶ write!("{:?}", self.0)

#[derive(Debug, Error)]
enum NetworkError {
    #[error("Connection timed out")] // 实现 Display trait
    Timeout,
    #[error("Unreachable")] // 实现 Display trait
    Unreachable,
}

#[derive(Debug, Error)]
enum LockError {
    #[error("Mechanical error: {0}")]
    MechanicalError(i32),
    #[error("Network error")]
    NetworkError(#[from] NetworkError), // 可以使用 #[from] 注解来实现 from trait
    #[error("Not authorized")]
    NotAuthorized,
}

fn main() {
    println!("lock error: {}", LockError::MechanicalError(1));
    println!(
        "lock error: {}",
        LockError::NetworkError(NetworkError::Timeout)
    );
    println!("lock error: {}", LockError::NotAuthorized);
}

fn lock_door() -> Result<(), LockError> {
    Err(NetworkError::Timeout)?
}
