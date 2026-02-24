use clap::Parser;
use tio::proto::DeviceRoute;
use tio::util;
use twinleaf::tio;

#[derive(Parser, Debug, Clone)]
pub struct TioOpts {
    /// Sensor root address (e.g., tcp://localhost, serial:///dev/ttyUSB0)
    #[arg(
        short = 'r',
        long = "root",
        default_value_t = util::default_proxy_url().to_string(),
        help = "Sensor root address"
    )]
    pub root: String,

    /// Sensor path in the sensor tree (e.g., /, /0, /0/1)
    #[arg(
        short = 's',
        long = "sensor",
        default_value = "/",
        help = "Sensor path in the sensor tree"
    )]
    pub route_path: String,
}

impl TioOpts {
    pub fn parse_route(&self) -> DeviceRoute {
        DeviceRoute::from_str(&self.route_path).unwrap_or_else(|_| DeviceRoute::root())
    }
}

#[derive(Parser, Debug)]
#[command(name = "tio-monitor", version, about = "Display live sensor data")]
pub struct MonitorCli {
    #[command(flatten)]
    pub tio: TioOpts,
    #[arg(short = 'a', long = "all")]
    pub all: bool,
    #[arg(long = "fps", default_value_t = 20)]
    pub fps: u32,
    #[arg(short = 'c', long = "colors")]
    pub colors: Option<String>,
}
