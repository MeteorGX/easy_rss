# future_rss

Use Rust to Serialize the Rss structure.

### Usage

```toml
future_rss = "*"
```

### Examples

##### Parse Xml

```rust
use future_rss::*;

fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address,"utf8")?;
    parser.author_tag = String::from("dc:creator");
    
    let rss = parser.parse_vec()?;
    println!("{:?}",rss);    
    Ok(())
}
```
#### Parse Web XMl

```rust
use future_rss::RssParser;

fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address,"utf8")?;
    parser.author_tag = String::from("dc:creator");
    assert!(parser.parse_json().is_ok());
    Ok(())
}
```

#### RSS To Json

```rust
use future_rss::RssParser;

fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address,"utf8")?;
    parser.author_tag = String::from("dc:creator");
    assert!(parser.parse_json().is_ok());
    Ok(())
}
```

### Rss Request Builder

```rust
use future_rss::RssParser;

fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::new();
    parser.author_tag = "dc:creator".into();
    parser.publish_tag = "pubDate".into();
    let xml = parser.request_xml(address.as_str(),charset.as_str())?;
    parser.set_xml(xml);
    assert!(parser.parse_vec().is_ok());
    Ok(())
}
```

### Advanced

[Examples](https://github.com/MeteorGX/future_rss_examples)
