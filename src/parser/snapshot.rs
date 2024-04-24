pub struct Snapshot
{
    start: usize,
}

impl Snapshot
{
    pub fn new(start: usize) -> Self
    {
        Self { start }
    }

    pub fn get_start_pos(&self) -> usize
    {
        return self.start;
    }

    pub fn to_string(&self) -> String
    {
        return self.start.to_string();
    }
}

