#[derive(Default)]
pub(crate) struct Threshold {
    target_min: (u8, u8, u8),
    target_max_channel: u8,
    pointer_minimum_channel: u8,
}

#[derive(Default)]
pub(crate) struct ThresholdBuilder {
    target_min: (u8, u8, u8),
    target_max_channel: u8,
    pointer_minimum_channel: u8,
}

impl ThresholdBuilder {
    pub(crate) fn target_min(&mut self, target_min: (u8, u8, u8)) -> &mut Self {
        self.target_min = target_min;
        self
    }

    pub(crate) fn target_max_channel(&mut self, target_max_channel: u8) -> &mut Self {
        self.target_max_channel = target_max_channel;
        self
    }

    pub(crate) fn pointer_minimum_channel(&mut self, pointer_minimum_channel: u8) -> &mut Self {
        self.pointer_minimum_channel = pointer_minimum_channel;
        self
    }

    pub(crate) fn build(&self) -> Threshold {
        Threshold {
            target_min: self.target_min,
            target_max_channel: self.target_max_channel,
            pointer_minimum_channel: self.pointer_minimum_channel,
        }
    }
}

impl Threshold {
    pub(crate) fn builder() -> ThresholdBuilder {
        ThresholdBuilder::default()
    }

    pub(crate) fn target_min(&self) -> (u8, u8, u8) {
        self.target_min
    }

    pub(crate) fn target_max_channel(&self) -> u8 {
        self.target_max_channel
    }

    pub(crate) fn pointer_minimum_channel(&self) -> u8 {
        self.pointer_minimum_channel
    }
}
