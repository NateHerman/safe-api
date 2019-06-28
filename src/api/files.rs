// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::xorurl::{xorname_to_xorurl, xorurl_to_xorname, XorUrl};
use super::{BlsKeyPair, Safe};
use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use threshold_crypto::SecretKey;
use unwrap::unwrap;

// Each FileItem contains file metadata and the link to the file's ImmutableData XOR-URL
pub type FileItem = BTreeMap<String, String>;

// To use for mapping files names (with path in a flattened hierarchy) to FileItems
pub type FilesMap = BTreeMap<String, FileItem>;

// Type tag to use for the FilesContainer stored on AppendOnlyData
const FILES_CONTAINER_TYPE_TAG: u64 = 10_100;

impl Safe {
    /// # Create a map of paths to xorurls
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use safe_cli::Safe;
    /// # use unwrap::unwrap;
    /// # use std::collections::BTreeMap;
    /// # let mut safe = Safe::new("base32".to_string());
    /// let top = b"Something top level";
    /// let top_xorurl = safe.files_put_published_immutable(top).unwrap();
    /// let second = b"Something second level";
    /// let second_xorurl = safe.files_put_published_immutable(second).unwrap();
    /// let mut content_map = BTreeMap::new();
    /// content_map.insert("./tests/testfolder/test.md".to_string(), top_xorurl);
    /// content_map.insert("./tests/testfolder/subfolder/subexists.md".to_string(), second_xorurl);
    /// let file_map = safe.files_map_create( &content_map ).unwrap();
    /// # assert_eq!(true, file_map.contains("\"md\""));
    /// # assert_eq!(true, file_map.contains("\"./tests/testfolder/test.md\""));
    /// ```
    pub fn files_map_create(
        &mut self,
        content: &BTreeMap<String, String>,
    ) -> Result<String, String> {
        let mut files_map = FilesMap::default();
        let now = Utc::now().to_string().to_string();

        for (key, value) in content.iter() {
            let mut file_item = FileItem::new();
            let metadata = fs::metadata(&key).map_err(|err| {
                format!(
                    "Couldn't obtain metadata information for local file: {:?}",
                    err,
                )
            })?;

            file_item.insert("link".to_string(), value.to_string());

            let file_type = Path::new(&key).extension().ok_or("unknown")?;
            file_item.insert(
                "type".to_string(),
                file_type.to_str().unwrap_or_else(|| "unknown").to_string(),
            );

            let file_size = &metadata.len().to_string();
            file_item.insert("size".to_string(), file_size.to_string());

            // file_item.insert("permissions", metadata.permissions().to_string());
            file_item.insert("modified".to_string(), now.clone());
            file_item.insert("created".to_string(), now.clone());

            debug!("FileItem item: {:?}", file_item);

            &files_map.insert(key.to_string(), file_item);
        }

        // TODO: use RDF format and serialise it
        let serialised_rdf = serde_json::to_string(&files_map)
            .map_err(|err| format!("Couldn't serialise the FilesMap generated: {:?}", err))?;

        Ok(serialised_rdf)
    }

    /// # Create versioned data.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use safe_cli::Safe;
    /// # use unwrap::unwrap;
    /// # use std::collections::BTreeMap;
    /// # let mut safe = Safe::new("base32".to_string());
    /// let top = b"Something top level";
    /// let top_xorurl = safe.files_put_published_immutable(top).unwrap();
    /// let second = b"Something second level";
    /// let second_xorurl = safe.files_put_published_immutable(second).unwrap();
    /// let mut content_map = BTreeMap::new();
    /// content_map.insert("./tests/testfolder/test.md".to_string(), top_xorurl);
    /// content_map.insert("./tests/testfolder/subfolder/subexists.md".to_string(), second_xorurl);
    /// let file_map = safe.files_map_create( &content_map ).unwrap();
    /// # assert!(file_map.contains("\"md\""));
    /// # assert!(file_map.contains("\"./tests/testfolder/test.md\""));
    /// let xor_url = safe.files_container_create(file_map.into_bytes().to_vec() ).unwrap();
    /// assert!(xor_url.contains("safe://"))
    /// ```
    pub fn files_container_create(&mut self, files_map_data: Vec<u8>) -> Result<XorUrl, String> {
        // The FilesContainer is created as a AppendOnlyData with a single entry containing the
        // timestamp as the entry's key, and the serialised FilesMap as the entry's value
        let now = Utc::now().to_string().to_string();
        let files_container_data = vec![(now.into_bytes().to_vec(), files_map_data)];

        // Store the FilesContainer in a Published AppendOnlyData
        let xorname = self.safe_app.put_seq_appendable_data(
            files_container_data,
            None,
            FILES_CONTAINER_TYPE_TAG,
            None,
        )?;

        xorname_to_xorurl(&xorname, &self.xorurl_base)
    }

    pub fn files_container_get_latest(&self, xorurl: &str) -> Result<FilesMap, String> {
        let xorname = xorurl_to_xorname(xorurl)?;
        match self
            .safe_app
            .get_seq_appendable_latest(xorname, FILES_CONTAINER_TYPE_TAG)
        {
            Ok(latest_entry) => {
                // TODO: use RDF format and deserialise it
                let files_map =
                    serde_json::from_str(&String::from_utf8_lossy(&latest_entry.1.as_slice()))
                        .map_err(|err| {
                            format!(
                        "Couldn't deserialise the FilesMap stored in the FilesContainer: {:?}",
                        err
                    )
                        })?;
                Ok(files_map)
            }
            Err("SeqAppendOnlyDataEmpty") => Ok(FilesMap::default()),
            Err("SeqAppendOnlyDataNotFound") | Err(_) => {
                Err("No FilesContainer found at this address".to_string())
            }
        }
    }

    #[allow(dead_code)]
    pub fn files_container_update(
        &mut self,
        xorurl: &str,
        files_map_data: Vec<u8>,
    ) -> Result<u64, String> {
        // The FilesContainer is updated by adding an entry containing the timestamp as the
        // entry's key, and the serialised new version of the FilesMap as the entry's value
        let now = Utc::now().to_string().to_string();
        let files_container_data = (now.into_bytes().to_vec(), files_map_data);

        let xorname = xorurl_to_xorname(xorurl)?;

        // Append new entry in the FilesContainer, which is a Published AppendOnlyData
        let new_version = self.safe_app.append_seq_appendable_data(
            files_container_data,
            xorname,
            FILES_CONTAINER_TYPE_TAG,
        )?;

        Ok(new_version)
    }

    // TODO:
    // Upload files as ImmutableData
    // Check if file or dir
    // if dir, grab and do many.
    // upload individual file
    // get file metadata?
    // if not now... when?

    /// # Put Published ImmutableData
    /// Put data blobs onto the network.
    ///
    /// ## Example
    /// ```
    /// # use safe_cli::Safe;
    /// # use unwrap::unwrap;
    /// # let mut safe = Safe::new("base32".to_string());
    /// let data = b"Something super good";
    /// let xorurl = safe.files_put_published_immutable(data).unwrap();
    /// # let received_data = safe.files_get_published_immutable(&xorurl).unwrap();
    /// # assert_eq!(received_data, data);
    /// ```
    pub fn files_put_published_immutable(&mut self, data: &[u8]) -> Result<XorUrl, String> {
        // TODO: do we want ownership from other PKs yet?
        let xorname = self.safe_app.files_put_published_immutable(&data)?;

        xorname_to_xorurl(&xorname, &self.xorurl_base)
    }

    /// # Get Published ImmutableData
    /// Put data blobs onto the network.
    ///
    /// ## Example
    /// ```
    /// # use safe_cli::Safe;
    /// # use unwrap::unwrap;
    /// # let mut safe = Safe::new("base32".to_string());
    /// # let data = b"Something super good";
    /// let xorurl = safe.files_put_published_immutable(data).unwrap();
    /// let received_data = safe.files_get_published_immutable(&xorurl).unwrap();
    /// # assert_eq!(received_data, data);
    /// ```
    pub fn files_get_published_immutable(&self, xorurl: &str) -> Result<Vec<u8>, String> {
        // TODO: do we want ownership from other PKs yet?
        let xorname = xorurl_to_xorname(&xorurl)?;
        self.safe_app.files_get_published_immutable(xorname)
    }
}

// Unit Tests

#[test]
fn test_keys_create_preload_test_coins() {}