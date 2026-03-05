use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRecord {
    pub tick: u64,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub inputs: Vec<InputRecord>,
}

pub struct Recorder {
    inputs: Vec<InputRecord>,
    last_left: bool,
    last_right: bool,
}

impl Recorder {
    pub fn new() -> Self {
        Recorder {
            inputs: Vec::new(),
            last_left: false,
            last_right: false,
        }
    }

    pub fn record(&mut self, tick: u64, left: bool, right: bool) {
        if left != self.last_left || right != self.last_right {
            self.inputs.push(InputRecord { tick, left, right });
            self.last_left = left;
            self.last_right = right;
        }
    }

    pub fn finish(self) -> Recording {
        Recording {
            inputs: self.inputs,
        }
    }
}

pub struct GhostPlayer {
    recording: Recording,
    cursor: usize,
    pub left: bool,
    pub right: bool,
}

impl GhostPlayer {
    pub fn new(recording: Recording) -> Self {
        GhostPlayer {
            recording,
            cursor: 0,
            left: false,
            right: false,
        }
    }

    pub fn update(&mut self, tick: u64) {
        while self.cursor < self.recording.inputs.len()
            && self.recording.inputs[self.cursor].tick <= tick
        {
            let rec = &self.recording.inputs[self.cursor];
            self.left = rec.left;
            self.right = rec.right;
            self.cursor += 1;
        }
    }

    pub fn is_done(&self) -> bool {
        self.cursor >= self.recording.inputs.len() && !self.left && !self.right
    }
}

pub fn save_recording(recording: &Recording, path: &Path) -> std::io::Result<()> {
    let json = serde_json::to_string(recording)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, json)
}

pub fn load_recording(path: &Path) -> std::io::Result<Recording> {
    let json = fs::read_to_string(path)?;
    let recording: Recording = serde_json::from_str(&json)?;
    Ok(recording)
}

pub fn load_all_recordings(dir: &Path) -> Vec<Recording> {
    let mut recordings = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(rec) = load_recording(&path) {
                    recordings.push(rec);
                }
            }
        }
    }
    recordings
}
