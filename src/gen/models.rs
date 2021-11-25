use crate::{http::*, xml::xml_to_data};
use aws_sdk_s3::model::*;
use aws_smithy_xml::decode::{Document, ScopedDecoder};
use hyper::body::Bytes;

pub fn parse_create_bucket_configuration(xml: Bytes) -> Result<CreateBucketConfiguration, S3Error> {
    let mut b = CreateBucketConfiguration::builder();
    let mut doc = Document::try_from(&xml[..]).unwrap();
    let mut d = doc.root_element().unwrap();
    if !d.start_el().matches("CreateBucketConfiguration") {
        Err(S3Error::bad_request("Bad CreateBucketConfiguration"))?;
    }
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("LocationConstraint") => {
                b = b.set_location_constraint(xml_to_data(&mut d));
            }
            _ => Err(S3Error::bad_request("Bad CreateBucketConfiguration"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_delete_objects(xml: Bytes) -> Result<Delete, S3Error> {
    let mut b = Delete::builder();
    let mut doc = Document::try_from(&xml[..]).unwrap();
    let mut d = doc.root_element().unwrap();
    if !d.start_el().matches("Delete") {
        Err(S3Error::bad_request("Bad Delete"))?;
    }
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("Quiet") => {
                b = b.set_quiet(xml_to_data(&mut d));
            }
            el if el.matches("Object") => {
                b = b.objects(parse_object_identifier(&mut d)?);
            }
            _ => Err(S3Error::bad_request("Bad Delete"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_object_identifier(d: &mut ScopedDecoder) -> Result<ObjectIdentifier, S3Error> {
    let mut b = ObjectIdentifier::builder();
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("Key") => b = b.set_key(xml_to_data(&mut d)),
            el if el.matches("VersionId") => b = b.set_version_id(xml_to_data(&mut d)),
            _ => {
                return Err(S3Error::bad_request("Bad ObjectIdentifier"));
            }
        };
    }
    Ok(b.build())
}

pub fn parse_complete_multipart_upload(xml: Bytes) -> Result<CompletedMultipartUpload, S3Error> {
    let mut b = CompletedMultipartUpload::builder();
    let mut doc = Document::try_from(&xml[..]).unwrap();
    let mut d = doc.root_element().unwrap();
    if !d.start_el().matches("CompleteMultipartUpload") {
        Err(S3Error::bad_request("Bad CompleteMultipartUpload"))?;
    }
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("Part") => b = b.parts(parse_completed_part(&mut d)?),
            _ => Err(S3Error::bad_request("Bad CompleteMultipartUpload"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_completed_part(d: &mut ScopedDecoder) -> Result<CompletedPart, S3Error> {
    let mut b = CompletedPart::builder();
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("Etag") => b = b.set_e_tag(xml_to_data(&mut d)),
            el if el.matches("PartNumber") => b = b.set_part_number(xml_to_data(&mut d)),
            _ => Err(S3Error::bad_request("Bad CompletedPart"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_access_control_policy(xml: Bytes) -> Result<AccessControlPolicy, S3Error> {
    let mut b = AccessControlPolicy::builder();
    let mut doc = Document::try_from(&xml[..]).unwrap();
    let mut d = doc.root_element().unwrap();
    if !d.start_el().matches("AccessControlPolicy") {
        Err(S3Error::bad_request("Bad AccessControlPolicy"))?;
    }
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("AccessControlList") => {
                while let Some(mut d) = d.next_tag() {
                    match d.start_el() {
                        el if el.matches("Grant") => b = b.grants(parse_grant(&mut d)?),
                        _ => Err(S3Error::bad_request("Bad AccessControlPolicy"))?,
                    };
                }
            }
            el if el.matches("Owner") => b = b.owner(parse_owner(&mut d)?),
            _ => Err(S3Error::bad_request("Bad AccessControlPolicy"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_owner(d: &mut ScopedDecoder) -> Result<Owner, S3Error> {
    let mut b = Owner::builder();
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("ID") => b = b.set_id(xml_to_data(&mut d)),
            el if el.matches("DisplayName") => b = b.set_display_name(xml_to_data(&mut d)),
            _ => Err(S3Error::bad_request("Bad Owner"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_grant(d: &mut ScopedDecoder) -> Result<Grant, S3Error> {
    let mut b = Grant::builder();
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("Grantee") => b = b.grantee(parse_grantee(&mut d)?),
            el if el.matches("Permission") => b = b.set_permission(xml_to_data(&mut d)),
            _ => Err(S3Error::bad_request("Bad Grant"))?,
        };
    }
    Ok(b.build())
}

pub fn parse_grantee(d: &mut ScopedDecoder) -> Result<Grantee, S3Error> {
    let mut b = Grantee::builder();
    while let Some(mut d) = d.next_tag() {
        match d.start_el() {
            el if el.matches("DisplayName") => b = b.set_display_name(xml_to_data(&mut d)),
            el if el.matches("EmailAddress") => b = b.set_email_address(xml_to_data(&mut d)),
            el if el.matches("ID") => b = b.set_id(xml_to_data(&mut d)),
            el if el.matches("Type") => b = b.set_type(xml_to_data(&mut d)),
            el if el.matches("URI") => b = b.set_uri(xml_to_data(&mut d)),
            _ => Err(S3Error::bad_request("Bad Grantee"))?,
        };
    }
    Ok(b.build())
}
