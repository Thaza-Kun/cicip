use chrono::{DateTime, Utc};
use serde::Deserialize;
use chrono::serde::ts_microseconds;



#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Facebook {
    #[serde(with="ts_microseconds")]
    timestamp: DateTime<Utc>,
    data: Vec<FacebookData>,
    title: Option<String>,
    attachments: Option<Vec<FacebookAttachment>>,
    tags: Option<Vec<FacebookTag>>,
}
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookTag {
    name: String
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookData {
    post: Option<String>,
    // #[serde(with="ts_microseconds")]
    update_timestamp: Option<u128>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookAttachment {
    data: Vec<FacebookAttachmentData>
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookAttachmentData {
    external_context: Option<FacebookAttachmentDataExternalContext>,
    media: Option<FacebookMedia>,
    place: Option<FacebookPlaceData>,
    event: Option<FacebookEvent>,
    note: Option<FacebookNote>,
    text: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookNote {
    title: String,
    text: String,
    cover_photo: FacebookCoverPhoto,
    #[serde(with="ts_microseconds")]
    created_timestamp: DateTime<Utc>,
    #[serde(with="ts_microseconds")]
    updated_timestamp: DateTime<Utc>,
    tags: Vec<FacebookTag>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookCoverPhoto {
    uri: String,
    #[serde(with="ts_microseconds")]
    creation_timestamp: DateTime<Utc>,
    media_metadata: FacebookMediaMetadata,
    title: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookEvent {
    name: String,
    #[serde(with="ts_microseconds")]
    start_timestamp: DateTime<Utc>,
    #[serde(with="ts_microseconds")]
    end_timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookPlaceData {
    name: String,
    coordinate: FacebookPlaceDataCoordinate,
    address: String,
    url: String
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookPlaceDataCoordinate {
    latitude: f64,
    longitude: f64
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookMedia {
    title: Option<String>,
    description: Option<String>,
    uri: String,
    #[serde(with="ts_microseconds")]
    creation_timestamp: DateTime<Utc>,
    media_metadata: Option<FacebookMediaMetadata>,
    thumbnail: Option<FacebookThumbnail>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookThumbnail {
    uri: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookMediaMetadata {
    photo_metadata: Option<FacebookPhotoMetadata>,
    video_metadata: Option<FacebookVideoMetadata>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookPhotoMetadata {
    exif_data: Vec<FacebookExifData>
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookVideoMetadata {
    exif_data: Vec<FacebookExifData>
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookExifData {
    taken_timestamp: Option<u128>,
    upload_timestamp: Option<u128>,
    modified_timestamp: Option<u128>,
    upload_ip: Option<String>,
    iso: Option<u128>,
    focal_length: Option<String>,
    camera_make: Option<String>,
    camera_model: Option<String>,
    exposure: Option<String>,
    orientation: Option<u8>,
    original_width: Option<f64>,
    original_height: Option<f64>,
    f_stop: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FacebookAttachmentDataExternalContext {
    url: Option<String>,
    name: Option<String>,
    source: Option<String>,
}

