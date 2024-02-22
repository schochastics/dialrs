use phonenumber::Type;
use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::FixedLine => "FixedLine",
                Type::Mobile => "Mobile",
                Type::FixedLineOrMobile => "FixedLineOrMobile",
                Type::TollFree => "TollFree",
                Type::PremiumRate => "PremiumRate",
                Type::SharedCost => "SharedCost",
                Type::PersonalNumber => "PersonalNumber",
                Type::Voip => "Voip",
                Type::Pager => "Pager",
                Type::Uan => "Uan",
                Type::Emergency => "Emergency",
                Type::Voicemail => "Voicemail",
                Type::ShortCode => "ShortCode",
                Type::StandardRate => "StandardRate",
                Type::Carrier => "Carrier",
                Type::NoInternational => "NoInternational",
                Type::Unknown => "Unknown",
            }
        )
    }
}
