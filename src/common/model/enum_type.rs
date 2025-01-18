#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GlueType {
    Bean,
    GlueGroovy,
    GlueShell,
    GluePython,
    GluePhp,
    GlueNodejs,
    GluePowerShell,
}

impl GlueType {
    /// from:
    ///     BEAN,
    ///     GLUE_GROOVY,
    ///     GLUE_SHELL,
    ///     GLUE_PYTHON,
    ///     GLUE_PHP,
    ///     GLUE_NODEJS,
    ///     GLUE_POWERSHELL,
    pub fn from_str(glue_type: &str) -> Option<GlueType> {
        match glue_type {
            "BEAN" => Some(GlueType::Bean),
            "GLUE_GROOVY" => Some(GlueType::GlueGroovy),
            "GLUE_SHELL" => Some(GlueType::GlueShell),
            "GLUE_PYTHON" => Some(GlueType::GluePython),
            "GLUE_PHP" => Some(GlueType::GluePhp),
            "GLUE_NODEJS" => Some(GlueType::GlueNodejs),
            "GLUE_POWERSHELL" => Some(GlueType::GluePowerShell),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            GlueType::Bean => "BEAN",
            GlueType::GlueGroovy => "GLUE_GROOVY",
            GlueType::GlueShell => "GLUE_SHELL",
            GlueType::GluePython => "GLUE_PYTHON",
            GlueType::GluePhp => "GLUE_PHP",
            GlueType::GlueNodejs => "GLUE_NODEJS",
            GlueType::GluePowerShell => "GLUE_POWERSHELL",
        }
    }

    pub fn is_script(&self) -> bool {
        match self {
            GlueType::Bean | GlueType::GlueGroovy => false,
            _ => true,
        }
    }

    pub fn get_cmd(&self) -> &str {
        match self {
            GlueType::Bean | GlueType::GlueGroovy => "",
            GlueType::GlueShell => "bash",
            GlueType::GluePython => "python",
            GlueType::GluePhp => "php",
            GlueType::GlueNodejs => "node",
            GlueType::GluePowerShell => "powershell",
        }
    }

    pub fn get_suffix(&self) -> &str {
        match self {
            GlueType::Bean | GlueType::GlueGroovy => "",
            GlueType::GlueShell => ".sh",
            GlueType::GluePython => ".py",
            GlueType::GluePhp => ".php",
            GlueType::GlueNodejs => ".js",
            GlueType::GluePowerShell => ".ps1",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutorBlockStrategy {
    SerialExecution,
    DiscardLater,
    CoverEarly,
    Other,
}

impl ExecutorBlockStrategy {
    pub fn from_str(s: &str) -> ExecutorBlockStrategy {
        match s {
            "SERIAL_EXECUTION" => ExecutorBlockStrategy::SerialExecution,
            "DISCARD_LATER" => ExecutorBlockStrategy::DiscardLater,
            "COVER_EARLY" => ExecutorBlockStrategy::CoverEarly,
            _ => ExecutorBlockStrategy::Other,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ExecutorBlockStrategy::SerialExecution => "SERIAL_EXECUTION",
            ExecutorBlockStrategy::DiscardLater => "DISCARD_LATER",
            ExecutorBlockStrategy::CoverEarly => "COVER_EARLY",
            ExecutorBlockStrategy::Other => "OTHER",
        }
    }
}
