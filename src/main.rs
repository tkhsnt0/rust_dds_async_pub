use anyhow::Result;
use rand::{self, Rng};
use rustdds::{DomainParticipantBuilder, Keyed, QosPolicyBuilder, TopicKind};
use serde::{Deserialize, Serialize};
use tokio;
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SensorStatus {
    sensor_type: String,
    frequeny: u32,
    power: u32,
    squelti: u32,
}
impl Keyed for SensorStatus {
    type K = String;
    fn key(&self) -> Self::K {
        self.sensor_type.clone()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SensorBit {
    sensor_type: String,
    bit_on: bool,
}
impl Keyed for SensorBit {
    type K = String;
    fn key(&self) -> Self::K {
        self.sensor_type.clone()
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("Start Publisher");
    let dp = DomainParticipantBuilder::new(0).build()?;
    let qos = QosPolicyBuilder::new().build();
    let topic_sensor_config = dp.create_topic(
        "SensorStatus".to_string(),
        "this is test topic1".to_string(),
        &qos,
        TopicKind::WithKey,
    )?;
    let topic_sensor_bit = dp.create_topic(
        "SensorBit".to_string(),
        "this is test topic2".to_string(),
        &qos,
        TopicKind::WithKey,
    )?;
    let publisher = dp.create_publisher(&qos)?;
    let writer_sensor_config =
        publisher.create_datawriter_cdr::<SensorStatus>(&topic_sensor_config, Some(qos.clone()))?;
    let writer_sensor_bit =
        publisher.create_datawriter_cdr::<SensorBit>(&topic_sensor_bit, Some(qos.clone()))?;

    for cnt in 0.. {
        let config = SensorStatus {
            sensor_type: "radio".to_string(),
            frequeny: rand::thread_rng().gen_range(2000000..29999999),
            power: rand::thread_rng().gen_range(100..400),
            squelti: rand::thread_rng().gen_range(0..255),
        };
        let bit = SensorBit {
            sensor_type: "radio".to_string(),
            bit_on: true,
        };
        writer_sensor_config.async_write(config, None).await?;
        writer_sensor_bit.async_write(bit, None).await?;
        println!("publish: {}", cnt);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    Ok(())
}
