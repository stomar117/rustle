pub struct ReadVector {
    token_vector: Option<Vec<char>>
}

impl ReadVector {
    pub fn new () -> Self {
        Self {
            token_vector: Some(Vec::new())
        }
    }

    pub fn join(&self) -> Result<String, &str> {
        let mut token_string: String = String::new();
        match &self.token_vector {
            Some(vector) => {
                for token in vector {
                    token_string.push(*token);
                }
                Ok(token_string)
            }
            None => Err("Please check if the readvector struct was constructed")
        }
    }

    pub fn push(&mut self, token: char) -> Result<(), &str> {
        match &mut self.token_vector {
            Some(vector) => Ok((*vector).push(token)),
            None => Err("Please check if the readvector struct was constructed")
        }
    }

    pub fn del(&mut self) -> Result<(), &str> {
        match &mut self.token_vector {
            Some(vector) => {
                vector.pop();
                Ok(())
            },
            None => Err("Please check if the readvector struct was constructed")
        }
    }
}