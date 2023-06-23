pub struct Snapshot
{
    start: usize,
}

impl Snapshot
{
    #[allow(dead_code)]
    pub fn new(start: usize) -> Self
    {
        Self { start }
    }

    #[allow(dead_code)]
    pub fn get_start_pos(&self) -> usize
    {
        return self.start;
    }

    pub fn to_string(&self) -> String
    {
        return self.start.to_string();
    }
}

