//!
//! Rss Parse Library
//!
//! ```
//! use future_rss::RssParser;
//!
//! fn main()->Result<(),Box<dyn std::error::Error>> {
//!     let address = "https://www.zhihu.com/rss";
//!     let mut parser = RssParser::from_url(address,"utf8")?;
//!     parser.author_tag = String::from("dc:creator");
//!     let rss = parser.parse_vec()?;
//!     println!("{:?}",rss);
//!     Ok(())
//! }
//! ```

use std::io::prelude::*;
use std::fs::File;
use quick_xml::Reader;
use quick_xml::events::Event;
use json::{object,array};

/// &lt;item&gt;&lt;/item&gt;
pub static RSS_DEFAULT_NODE_TAG:&'static str = "item";

/// &lt;title&gt;...&lt;/title&gt;
pub static RSS_DEFAULT_TITLE_TAG:&'static str = "title";

/// &lt;link&gt;...&lt;/link&gt;
pub static RSS_DEFAULT_LINK_TAG:&'static str = "link";

/// &lt;author&gt;...&lt;/author&gt;
pub static RSS_DEFAULT_AUTHOR_TAG:&'static str = "author";

/// &lt;description&gt;...&lt;/description&gt;
pub static RSS_DEFAULT_DESC_TAG:&'static str = "description";

/// &lt;guid&gt;...&lt;/guid&gt;
pub static RSS_DEFAULT_GUID_TAG:&'static str = "guid";

/// &lt;pubDate&gt;...&lt;/pubDate&gt;
pub static RSS_DEFAULT_PUBLISH_TAG:&'static str = "pubDate";

/// Check &lt;xml&gt; and &gt;rss&lt;
pub static XML_DEFAULT_TAG:&'static str = "xml";
pub static RSS_DEFAULT_TAG:&'static str = "rss";

///
/// Rss Item Node
///
/// ```
/// use future_rss::RssItem;
/// fn main(){
///     let item = RssItem::default();
///     println!("{:?}",item);
/// }
/// ```
#[derive(Debug)]
#[allow(dead_code)]
pub struct RssItem{
    pub title: String,
    pub link: String,
    pub author: String,
    pub description: String,
    pub guid: String,
    pub publish: String,
}

///
/// Rss Parse Utils
///
/// ### Parse Xml
/// ```
/// use future_rss::RssParser;
///
/// fn main()->Result<(),Box<dyn std::error::Error>>{
///     let mut parser_default = RssParser::new();
///     println!("{:?}",parser_default);
///     parser_default.set_xml(String::from(
///        r#"<?xml version="1.0" encoding="UTF-8" ?>
///         <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
///             <item>
///                 <title>Hey!</title>
///                 <link>examples.com</link>
///                 <description>hello.world!</description>
///                 <author>MeteorCat</author>
///                 <guid>unique key</guid>
///                 <pubDate>2020-05-28 15:00:00</pubDate>
///             </item>
///         </rss>
///         "#
///     ));
///     println!("{:?}",parser_default.parse_vec()?);
///     Ok(())
/// }
/// ```
///
/// ### Parse Web XMl
/// ```
/// use future_rss::RssParser;
///
/// fn main()->Result<(),Box<dyn std::error::Error>> {
///     let address = "https://www.zhihu.com/rss";
///     let mut parser = RssParser::from_url(address,"utf8")?;
///     parser.author_tag = String::from("dc:creator");
///     let rss = parser.parse_vec()?;
///     println!("{:?}",rss);
///     Ok(())
/// }
/// ```
///
/// ### RSS To Json
/// ```
/// use future_rss::RssParser;
///
/// fn main()->Result<(),Box<dyn std::error::Error>> {
///     let address = "https://www.zhihu.com/rss";
///     let mut parser = RssParser::from_url(address,"utf8")?;
///     parser.author_tag = String::from("dc:creator");
///     assert!(parser.parse_json().is_ok());
///     Ok(())
/// }
/// ```
///
/// ### Rss Request Builder
/// ```
/// use future_rss::RssParser;
///
/// fn main()->Result<(),Box<dyn std::error::Error>> {
///     let address = "https://www.zhihu.com/rss";
///     let charset = "utf8";
///     let mut parser = RssParser::new();
///     parser.author_tag = "dc:creator".into();
///     parser.publish_tag = "pubDate".into();
///     let xml = parser.request_xml(address,charset)?;
///     parser.set_xml(xml);
///     assert!(parser.parse_vec().is_ok());
///     Ok(())
/// }
/// ```
#[derive(Debug)]
#[allow(dead_code)]
pub struct RssParser{
    xml:String,
    pub node_tag:String,
    pub title_tag:String,
    pub link_tag:String,
    pub author_tag:String,
    pub description_tag:String,
    pub guid_tag:String,
    pub publish_tag:String,
}


impl Default for RssItem{
    fn default() -> Self {
        Self{
            title:String::new(),
            link:String::new(),
            author:String::new(),
            description:String::new(),
            guid:String::new(),
            publish:String::new()
        }
    }
}


impl RssParser{

    ///
    /// Todo: Optimization
    ///
    pub fn check_xml(&mut self)->bool{
        //todo: need optimization
        if !self.xml.contains(XML_DEFAULT_TAG) && !self.xml.contains(&XML_DEFAULT_TAG.to_uppercase()) {
            return false;
        }
        if !self.xml.contains(RSS_DEFAULT_TAG) && !self.xml.contains(&RSS_DEFAULT_TAG.to_uppercase()) {
            return false;
        }
        return true;
    }


    ///
    /// Request Rss by Web
    ///
    pub fn request_xml(&mut self,url:&str,charset:&str)->Result<String,reqwest::Error>{
        Ok(reqwest::blocking::get(url)?
            .text_with_charset(charset)?)
    }

    ///
    /// Request RSS by File
    pub async fn request_file(&mut self,filename:&str)->Result<String,std::io::Error>{
        let mut f = File::open(filename)?;
        let mut body = String::new();
        f.read_to_string(&mut body)?;
        Ok(body)
    }


    pub fn new()->Self{
        Self{
            xml:String::new(),
            node_tag:String::from(RSS_DEFAULT_NODE_TAG),
            title_tag:String::from(RSS_DEFAULT_TITLE_TAG),
            link_tag:String::from(RSS_DEFAULT_LINK_TAG),
            author_tag:String::from(RSS_DEFAULT_AUTHOR_TAG),
            description_tag:String::from(RSS_DEFAULT_DESC_TAG),
            guid_tag:String::from(RSS_DEFAULT_GUID_TAG),
            publish_tag:String::from(RSS_DEFAULT_PUBLISH_TAG)
        }
    }


    pub fn from_str(xml:String)->Result<Self,std::io::Error>{
        let mut parser = Self::new();
        parser.xml = xml;
        if !parser.check_xml() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed by RssParser::check_xml"
            ))
        }else {
            Ok(parser)
        }
    }



    pub fn from_url(url:&str,charset:&str)->Result<Self,std::io::Error>{
        let mut parser = Self::new();
        match parser.request_xml(url,charset) {
            Ok(body) => {
                parser.xml = body;
                if !parser.check_xml() {
                    Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
                }else {
                    Ok(parser)
                }
            }
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,e.to_string()))
        }
    }

    pub async fn from_file(filename:&str)->Result<Self,std::io::Error>{
        let mut parser = Self::new();
        let body = parser.request_file(filename).await?;

        parser.xml = body;
        if !parser.check_xml() {
            Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        }else {
            Ok(parser)
        }
    }

    pub fn parse_vec(&mut self)->Result<Vec<RssItem>,std::io::Error>{
        let mut reader = Reader::from_str(self.xml.as_str());

        reader.trim_text(true);
        reader.check_end_names(true);
        reader.check_comments(false);
        reader.expand_empty_elements(true);


        let mut buff = Vec::new();
        let mut nodes = Vec::new();
        let mut active = String::new();

        loop{
            match reader.read_event(&mut buff) {
                // Fetch = <Item></Item>
                Ok(Event::Start(ref e)) => {
                    active = std::str::from_utf8(e.name())
                        .expect("Failed By Parse <Item>")
                        .to_string();

                    if self.node_tag.eq_ignore_ascii_case(&active) {
                        nodes.push(RssItem::default());
                    }
                }

                // Fetch = <Item><Node><CDATA></Node><Item>
                Ok(Event::CData(ref e)) => {
                    let node_text = std::str::from_utf8(e.escaped())
                        .expect("Failed by Parse <CData>");

                    if let Some(last) = nodes.last_mut() {
                        match active {
                            _ if self.title_tag.eq_ignore_ascii_case(&active) => { last.title = node_text.to_string() },
                            _ if self.link_tag.eq_ignore_ascii_case(&active) => { last.link = node_text.to_string() },
                            _ if self.author_tag.eq_ignore_ascii_case(&active) => { last.author = node_text.to_string() },
                            _ if self.description_tag.eq_ignore_ascii_case(&active) => { last.description = node_text.to_string() },
                            _ if self.guid_tag.eq_ignore_ascii_case(&active) => { last.guid = node_text.to_string() },
                            _ if self.publish_tag.eq_ignore_ascii_case(&active) => { last.publish = node_text.to_string() },
                            _ => (),
                        }
                    }
                }

                // Fetch = <Item><Node></Node><Item>
                Ok(Event::Text(ref e)) => {
                    let node_text = e
                        .unescape_and_decode(&reader)
                        .expect("Failed by Parse <Node>");

                    if let Some(last) = nodes.last_mut() {
                        match active {
                            _ if self.title_tag.eq_ignore_ascii_case(&active) => { last.title = node_text.to_string() },
                            _ if self.link_tag.eq_ignore_ascii_case(&active) => { last.link = node_text.to_string() },
                            _ if self.author_tag.eq_ignore_ascii_case(&active) => { last.author = node_text.to_string() },
                            _ if self.description_tag.eq_ignore_ascii_case(&active) => { last.description = node_text.to_string() },
                            _ if self.guid_tag.eq_ignore_ascii_case(&active) => { last.guid = node_text.to_string() },
                            _ if self.publish_tag.eq_ignore_ascii_case(&active) => { last.publish = node_text.to_string() },
                            _ => (),
                        }
                    }
                }

                Ok(Event::Eof) => break,
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other,format!("{:?}",e))),
                _ =>(),
            }
            buff.clear();
        }

        Ok(nodes)
    }

    pub fn parse_json(&mut self)->Result<String,std::io::Error>{
        let item = self.parse_vec()?;
        let mut json = array![];
        for node in item.into_iter() {
            let data = object!{
                "title": node.title,
                "link": node.link,
                "author": node.author,
                "description": node.description,
                "guid": node.guid,
                "publish": node.publish,
            };
            json.push(data).expect("Failed by Parse Json")
        }

        Ok(json.dump())
    }


    pub fn set_xml(&mut self,xml:String){
        self.xml = xml;
    }

    pub fn get_xml(&self)->&String{
        &self.xml
    }
}




#[cfg(test)]
mod tests {
    use crate::RssParser;

    #[test]
    fn future_rss_works()->Result<(),Box<dyn std::error::Error>> {
        let address = "https://www.zhihu.com/rss";
        let mut parser = RssParser::from_url(address,"utf8")?;
        parser.author_tag = String::from("dc:creator");
        let rss = parser.parse_vec()?;
        assert!(rss.len()>0);
        Ok(())
    }

    #[test]
    fn future_rss_to_json(){
        let address = "https://www.zhihu.com/rss";
        let mut parser = RssParser::from_url(address,"utf8").unwrap();
        parser.author_tag = String::from("dc:creator");
        assert!(parser.parse_json().is_ok());
    }


    #[test]
    fn future_rss_builder(){
        let mut parser = RssParser::new();
        parser.set_xml(String::from(
            r#"<?xml version="1.0" encoding="UTF-8" ?>
                <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
                    <item>
                        <title>Hey!</title>
                        <link>examples.com</link>
                        <description>hello.world!</description>
                        <author>MeteorCat</author>
                        <guid>unique key</guid>
                        <pubDate>2020-05-28 15:00:00</pubDate>
                    </item>
                </rss>
        "#));
        let rss = parser.parse_vec().unwrap();
        assert!(rss.len()>0);
    }
}
