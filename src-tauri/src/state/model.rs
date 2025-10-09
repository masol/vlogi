use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
}

fn load() -> serde_json::Result<()> {
    let data = r#"[ 
        {"type": "Circle", "radius": 10},
        {"type": "Rect", "width": 3, "height": 4}
    ]"#;

    let shapes: Vec<Shape> = serde_json::from_str(data)?;
    for s in shapes {
        println!("{:?}", s);
    }
    Ok(())
}