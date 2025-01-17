#[derive(Debug)]
enum IpAddr {
    V4([u8; 4]),
    V6([&'static str; 8]),
}

#[derive(Debug)]
struct EnumError;

trait Decay: Sized {
    fn decay(value: IpAddr) -> Result<Self, EnumError>;
}

impl IpAddr {
    pub fn decay<T: Decay>(self) -> Result<T, EnumError> {
        Decay::decay(self)
    }
}

impl Decay for [u8; 4] {
    fn decay(value: IpAddr) -> Result<Self, EnumError> {
        match value {
            IpAddr::V4(v) => Ok(v),
            _ => Err(EnumError),
        }
    }
}

impl Decay for [&'static str; 8] {
    fn decay(value: IpAddr) -> Result<Self, EnumError> {
        match value {
            IpAddr::V6(v) => Ok(v),
            _ => Err(EnumError),
        }
    }
}

impl Decay for String {
    fn decay(value: IpAddr) -> Result<Self, EnumError> {
        match value {
            IpAddr::V4(v) => Ok(format!("{}:{}:{}:{}", v[0], v[1], v[2], v[3])),
            IpAddr::V6(v) => Ok(format!(
                "{}::{}::{}::{}::{}::{}::{}::{}",
                v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
            )),
        }
    }
}

fn main() {
   let ip = IpAddr::V4([0; 4]);
   println!("V4 = {:?}", ip.decay::<[u8; 4]>());
   
   let ip = IpAddr::V6(["0000"; 8]);
   println!("V6 = {:?}", ip.decay::<[&'static str; 8]>());

   let ip = IpAddr::V4([0; 4]);
   println!("V6 = {:?}", ip.decay::<String>())
}
