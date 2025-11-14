 // Import Keywords struct
use crate::error::Error;
use crate::index::index_struct::Index;

impl Index { // Make it an Index method
    pub async fn extract_keywords_command( // Renamed to avoid conflict with Index::extract_keywords
        &self, // Takes self
        query: String,
    ) -> Result<(), Error> {
        let keywords_struct = self.extract_keywords(query.as_str()).await?; // Call as Index method

        for keyword in keywords_struct.keywords.iter() { // Iterate over keywords field
            println!("{}", keyword);
        }

        for keyword in keywords_struct.extra.iter() { // Also print extra keywords
            println!("{}", keyword);
        }

        Ok(())
    }
}
