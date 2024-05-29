use actix_files::Files;

pub fn static_files() -> Files {
    Files::new("/", "/home/federico/Advanced/walle-visualizer/src/wwwroot").index_file("index.html")
}