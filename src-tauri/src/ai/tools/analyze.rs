use rig_core::completion::ToolDefinition;
use serde_json::json;

pub fn analyze_transcript_tool() -> ToolDefinition {
    ToolDefinition {
        name: "extract_epic_moments".to_string(),
        description: "Extract the most epic, high-energy gaming moments or funny/viral moments from a video transcript. You must strictly ignore all non-gameplay segments.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "moments": {
                    "type": "array",
                    "description": "List of epic/funny moments found in the transcript",
                    "items": {
                        "type": "object",
                        "properties": {
                            "start": {
                                "type": "number",
                                "description": "Start time in seconds (e.g. 12.5)"
                            },
                            "end": {
                                "type": "number",
                                "description": "End time in seconds (e.g. 45.0)"
                            },
                            "description": {
                                "type": "string",
                                "description": "Description of the moment"
                            }
                        },
                        "required": ["start", "end", "description"]
                    }
                }
            },
            "required": ["moments"]
        })
    }
}
