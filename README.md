# opsgenie_rust

Light alerting client library for opsgenie 
This libaray in WIP status so be aware that API can be rewritten. I have created this API because `swagger-cli` is not able to genereate rust lib that is compiling and it was faster for me implement needed functionality(Alert creating) on my own.

## How to use it?
```
extern crate opsgenie_rust;
use opsgenie_rust::*;
use std::thread;

fn main() {
    let alert_data = AlertData::new("Some message".to_string())
        .alias("with alias".to_string())
        .tags(vec!["certyficates".to_string(), "expiring".to_string()])
        .entity("www.domain.com".to_string())
        .source("alert-sourtce".to_string())
        .priority(opsgenie_rust::Priority::P4);
    
    let opsgenie = OpsGenie::new("XXXXX-XXXXX-XXXXX-XXXXXX-XXXXX".to_string());

    let mut alert_ops = opsgenie.alert(alert_data).unwrap();

    thread::sleep_ms(10000);

    alert_ops.close().unwrap();
}

```


## TODO:
 - Finish implementing alert API
 - Adding heatbeat API support
 - we will see
 