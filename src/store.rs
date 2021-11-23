use aws_sdk_s3::{error::*, input::*, output::*, ByteStream};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tokio_stream::{Stream, StreamExt};

type DataStreamItem = Result<hyper::body::Bytes, aws_smithy_http::byte_stream::Error>;

pub async fn put_object(mut i: PutObjectInput) -> Result<PutObjectOutput, PutObjectError> {
    warn!("-----------------------");
    warn!("------ S3D STORE ------");
    warn!("-----------------------");
    let fname = format!(
        ".s3d/{}/{}",
        i.bucket().unwrap(),
        urlencoding::encode(i.key().unwrap())
    );
    write_stream_to_file(fname.as_ref(), &mut i.body)
        .await
        .map_err(|e| PutObjectError::unhandled(e))?;
    Ok(PutObjectOutput::builder().e_tag("s3d-etag-todo").build())
}

async fn write_stream_to_file<T: Stream<Item = DataStreamItem> + Unpin>(
    fname: &str,
    stream: &mut T,
) -> Result<(), std::io::Error> {
    let mut file = File::create(fname).await.unwrap();
    while let Some(v) = stream.next().await {
        file.write_all(&v.unwrap()).await.unwrap();
    }
    file.flush().await.unwrap();
    file.sync_all().await.unwrap();
    file.shutdown().await?;
    Ok(())
}

// async fn read_stream_from_file<T: Stream<Item = DataStreamItem> + Unpin>(
//     fname: &str,
//     stream: &mut T,
// ) -> Result<(), std::io::Error> {
//     let mut file = File::open(fname).await.unwrap();
//     while let Some(v) = file.next().await {
//         stream.write_all(&v.unwrap()).await.unwrap();
//     }
//     Ok(())
// }

// pub async fn get_object<T: Stream<Item = DataStreamItem> + Unpin>() -> Result<T, anyhow::Error> {
//     let mut file = File::open("foo.txt").await.unwrap();
//     while let Some(v) = file.next().await {
//         file.write_all(&v.unwrap()).await.unwrap();
//     }
//     file.flush().await.unwrap();
//     file.sync_all().await.unwrap();
//     file.shutdown().await.unwrap();
// }
