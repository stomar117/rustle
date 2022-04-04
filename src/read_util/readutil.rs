pub struct ReadVector {
    token_vector: Option<Vec<char>>,
    token_vector_lt: Vec<char>,
    token_vector_rt: Vec<char>
}

impl ReadVector {
    pub fn new () -> Self {
        Self {
            token_vector: Some(Vec::new()),
            token_vector_lt: Vec::new(),
            token_vector_rt: Vec::new()
        }
    }

    pub fn join(&self) -> Result<String, &str> {
        // let mut token_string: String = String::new();
        match &self.token_vector {
            Some(_vector) => {
                Ok(format!("{}{}", self.join_lt(), self.join_rt()))
            }
            None => Err("Please check if the readvector struct was constructed")
        }
    }

    pub fn is_empty(&self) -> bool {
        let mut tmp1 = self.token_vector_lt.clone();
        let mut tmp2 = self.token_vector_rt.clone();
        tmp1.append(&mut tmp2);
        return  tmp1.is_empty();
    }

    pub fn join_lt(&self) -> String {
        let mut token_string = String::new();
        for token in &self.token_vector_lt{
            token_string.push(*token);
        }
        return token_string;
    }

    pub fn join_rt(&self) -> String {
        let mut token_string = String::new();
        let mut reversed_vector = self.token_vector_rt.clone();
        reversed_vector.reverse();
        for token in reversed_vector{
            token_string.push(token);
        }
        return  token_string;
    }

    pub fn move_back(&mut self) -> Result<(), &str> {
        match &self.token_vector {
            Some(_) => {
                match self.token_vector_lt.pop() {
                    Some(mov_token) => {
                        self.token_vector_rt.push(mov_token);
                    }
                    None => {}
                }
            },
            None => {
                return Err("Please check if readvector was constructed properly");
            }
        }
        Ok(())
    }

    pub fn move_front(&mut self) -> Result<(), &str> {
        match &self.token_vector {
            Some(_) => {
                match self.token_vector_rt.pop() {
                    Some(mov_token) => {
                        self.token_vector_lt.push(mov_token);
                    }
                    None => {}
                }
            },
            None => {
                return Err("Please check if readvector was constructed properly");
            }
        }
        Ok(())
    }

    pub fn push(&mut self, token: char) -> Result<(), &str> {
        match &mut self.token_vector {
            Some(_vector) => Ok((self.token_vector_lt).push(token)),
            None => Err("Please check if the readvector struct was constructed")
        }
    }

    pub fn del(&mut self) -> Result<(), &str> {
        match &mut self.token_vector {
            Some(_vector) => {
                self.token_vector_rt.pop();
                Ok(())
            },
            None => Err("Please check if the readvector struct was constructed")
        }
    }
    pub fn del_back(&mut self) -> Result<(), &str> {
        match &mut self.token_vector {
            Some(_vector) => {
                self.token_vector_lt.pop();
                Ok(())
            },
            None => Err("Please check if the readvector struct was constructed")
        }
    }
}