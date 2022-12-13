use crate::codegen::s3_types;
use aws_smithy_http_server::operation::OperationShape;
use std::{future::Future, net::SocketAddr, pin::Pin};



macro_rules! gen_s3_handler_fn {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](
                self,
                input: <s3_types::[<$op>] as OperationShape>::Input,
            ) -> TraitFuture<
                <s3_types::[<$op>] as OperationShape>::Output,
                <s3_types::[<$op>] as OperationShape>::Error,
            > {
                panic!("S3Server [<$op>] not implemented")
            }
        }
    };
}

#[async_trait::async_trait]
pub trait S3Handler: Clone + Sized + Send {
    s3_types::generate_ops_code!(gen_s3_handler_fn);
}


/// aws_sigv4, aws_sig_auth
pub trait AuthHandler: Clone + Sized + Send {
    fn verify_sig_auth(&self, req: &HttpRequest) -> anyhow::Result<()>;
}

pub trait NoAuth {}

impl<T: NoAuth> AuthHandler for T {
    fn verify_sig_auth(&self, req: &HttpRequest) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait S3InputParser: Clone + Sized + Send {
    fn parse(&self, req: &HttpRequest) -> anyhow::Result<s3_types::S3Ops>;
}



// if req.uri().path() == "/" {
//     // let mut op = s3_types::ListBuckets::default();
//     // op.receive(req);
//     // h.handle(&mut op);
//     let res = self.list_buckets(()).unwrap();
//     log::debug!("res {:?}", res);
//     Ok(HttpResponse::new(HttpBody::from(format!(
//         "<ListAllMyBucketsResult>
//             <Owner>
//                 <ID>1</ID>
//                 <DisplayName>User1</DisplayName>
//             </Owner>
//             <Buckets>
//                 <Bucket>
//                     <Name>lala</Name>
//                     <CreationDate>19820608T001122Z</CreationDate>
//                 </Bucket>
//             </Buckets>
//         </ListAllMyBucketsResult>"
//     ))))
// } else {
//     Err(anyhow::format_err!("todo"))
// }
