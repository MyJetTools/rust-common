use crate::country_code::CountryCode;

use super::*;

pub const USA_PREFIX: &str = "+1";

pub fn create_country_code_nanp() -> Vec<(&'static str, PhoneCountryCode)> {
    // North American Numbering Plan (NANP) countries sharing +1
    vec![
        ("+1-264", CountryCode::AIA.into()),
        ("+1-268", CountryCode::ATG.into()),
        ("+1-242", CountryCode::BHS.into()),
        ("+1-246", CountryCode::BRB.into()),
        ("+1-441", CountryCode::BMU.into()),
        ("+1-284", CountryCode::VGB.into()),
        ("+1-345", CountryCode::CYM.into()),
        ("+1-767", CountryCode::DMA.into()),
        ("+1-809", CountryCode::DOM.into()),
        ("+1-473", CountryCode::GRD.into()),
        ("+1-671", CountryCode::GUM.into()),
        ("+1-876", CountryCode::JAM.into()),
        ("+1-664", CountryCode::MSR.into()),
        ("+1-670", CountryCode::MNP.into()),
        ("+1-787", CountryCode::PRI.into()),
        ("+1-869", CountryCode::KNA.into()),
        ("+1-758", CountryCode::LCA.into()),
        ("+1-784", CountryCode::VCT.into()),
        ("+1-721", CountryCode::SXM.into()),
        ("+1-649", CountryCode::TCA.into()),
        ("+1-340", CountryCode::VIR.into()),
        ("+1-808", CountryCode::UMI.into()), // United States Minor Outlying Islands
    ]
}

pub fn create_2_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+211", CountryCode::SSD.into()),
        ("+212", CountryCode::MAR.into()),
        ("+212", CountryCode::ESH.into()), // Western Sahara (disputed territory, uses Morocco's code)
        ("+213", CountryCode::DZA.into()),
        ("+216", CountryCode::TUN.into()),
        ("+218", CountryCode::LBY.into()),
        ("+220", CountryCode::GMB.into()),
        ("+221", CountryCode::SEN.into()),
        ("+222", CountryCode::MRT.into()),
        ("+223", CountryCode::MLI.into()),
        ("+224", CountryCode::GIN.into()),
        ("+225", CountryCode::CIV.into()),
        ("+226", CountryCode::BFA.into()),
        ("+227", CountryCode::NER.into()),
        ("+228", CountryCode::TGO.into()),
        ("+229", CountryCode::BEN.into()),
        ("+230", CountryCode::MUS.into()),
        ("+231", CountryCode::LBR.into()),
        ("+232", CountryCode::SLE.into()),
        ("+233", CountryCode::GHA.into()),
        ("+234", CountryCode::NGA.into()),
        ("+235", CountryCode::TCD.into()),
        ("+236", CountryCode::CAF.into()),
        ("+237", CountryCode::CMR.into()),
        ("+238", CountryCode::CPV.into()),
        ("+239", CountryCode::STP.into()),
        ("+240", CountryCode::GNQ.into()),
        ("+241", CountryCode::GAB.into()),
        ("+242", CountryCode::COG.into()),
        ("+243", CountryCode::COD.into()),
        ("+244", CountryCode::AGO.into()),
        ("+245", CountryCode::GNB.into()),
        ("+246", CountryCode::IOT.into()),
        ("+247", CountryCode::SHN.into()),
        ("+248", CountryCode::SYC.into()),
        ("+249", CountryCode::SDN.into()),
        ("+250", CountryCode::RWA.into()),
        ("+251", CountryCode::ETH.into()),
        ("+252", CountryCode::SOM.into()),
        ("+253", CountryCode::DJI.into()),
        ("+254", CountryCode::KEN.into()),
        ("+255", CountryCode::TZA.into()),
        ("+256", CountryCode::UGA.into()),
        ("+257", CountryCode::BDI.into()),
        ("+258", CountryCode::MOZ.into()),
        ("+260", CountryCode::ZMB.into()),
        ("+261", CountryCode::MDG.into()),
        ("+262", CountryCode::REU.into()),
        ("+262", CountryCode::ATF.into()), // French Southern Territories
        ("+263", CountryCode::ZWE.into()),
        ("+264", CountryCode::NAM.into()),
        ("+265", CountryCode::MWI.into()),
        ("+266", CountryCode::LSO.into()),
        ("+267", CountryCode::BWA.into()),
        ("+268", CountryCode::SWZ.into()),
        ("+269", CountryCode::COM.into()),
        ("+290", CountryCode::SHN.into()), // Saint Helena
        ("+291", CountryCode::ERI.into()),
        ("+297", CountryCode::ABW.into()),
        ("+298", CountryCode::FRO.into()),
        ("+299", CountryCode::GRL.into()),
    ]
}

pub fn create_2_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+20", CountryCode::EGY.into()),
        ("+27", CountryCode::ZAF.into()),
    ]
}

pub fn create_3_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+350", CountryCode::GIB.into()),
        ("+351", CountryCode::PRT.into()),
        ("+352", CountryCode::LUX.into()),
        ("+353", CountryCode::IRL.into()),
        ("+354", CountryCode::ISL.into()),
        ("+355", CountryCode::ALB.into()),
        ("+356", CountryCode::MLT.into()),
        ("+357", CountryCode::CYP.into()),
        ("+358", CountryCode::FIN.into()),
        ("+358", CountryCode::ALA.into()), // Åland Islands
        ("+359", CountryCode::BGR.into()),
        ("+370", CountryCode::LTU.into()),
        ("+371", CountryCode::LVA.into()),
        ("+372", CountryCode::EST.into()),
        ("+373", CountryCode::MDA.into()),
        ("+374", CountryCode::ARM.into()),
        ("+375", CountryCode::BLR.into()),
        ("+376", CountryCode::AND.into()),
        ("+377", CountryCode::MCO.into()),
        ("+378", CountryCode::SMR.into()),
        ("+379", CountryCode::VAT.into()),
        ("+380", CountryCode::UKR.into()),
        ("+381", CountryCode::SRB.into()),
        ("+382", CountryCode::MNE.into()),
        ("+383", CountryCode::XKX.into()), // Kosovo
        ("+385", CountryCode::HRV.into()),
        ("+386", CountryCode::SVN.into()),
        ("+387", CountryCode::BIH.into()),
        ("+389", CountryCode::MKD.into()),
    ]
}

pub fn create_3_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+30", CountryCode::GRC.into()),
        ("+31", CountryCode::NLD.into()),
        ("+32", CountryCode::BEL.into()),
        ("+33", CountryCode::FRA.into()),
        ("+34", CountryCode::ESP.into()),
        ("+36", CountryCode::HUN.into()),
        ("+39", CountryCode::ITA.into()),
    ]
}

pub fn create_4_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+420", CountryCode::CZE.into()),
        ("+421", CountryCode::SVK.into()),
        ("+423", CountryCode::LIE.into()),
    ]
}

pub fn create_4_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+40", CountryCode::ROU.into()),
        ("+41", CountryCode::CHE.into()),
        ("+43", CountryCode::AUT.into()),
        ("+44", CountryCode::GBR.into()),
        ("+44", CountryCode::GGY.into()), // Guernsey
        ("+44", CountryCode::JEY.into()), // Jersey
        ("+44", CountryCode::IMN.into()), // Isle of Man
        ("+45", CountryCode::DNK.into()),
        ("+46", CountryCode::SWE.into()),
        ("+47", CountryCode::NOR.into()),
        ("+47", CountryCode::SJM.into()), // Svalbard and Jan Mayen
        ("+48", CountryCode::POL.into()),
        ("+49", CountryCode::DEU.into()),
    ]
}

pub fn create_5_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+500", CountryCode::FLK.into()),
        ("+500", CountryCode::SGS.into()), // South Georgia and the South Sandwich Islands
        ("+501", CountryCode::BLZ.into()),
        ("+502", CountryCode::GTM.into()),
        ("+503", CountryCode::SLV.into()),
        ("+504", CountryCode::HND.into()),
        ("+505", CountryCode::NIC.into()),
        ("+506", CountryCode::CRI.into()),
        ("+507", CountryCode::PAN.into()),
        ("+508", CountryCode::SPM.into()),
        ("+509", CountryCode::HTI.into()),
        ("+590", CountryCode::GLP.into()),
        ("+590", CountryCode::MAF.into()), // Saint Martin
        ("+590", CountryCode::BLM.into()), // Saint Barthélemy
        ("+591", CountryCode::BOL.into()),
        ("+592", CountryCode::GUY.into()),
        ("+593", CountryCode::ECU.into()),
        ("+594", CountryCode::GUF.into()),
        ("+595", CountryCode::PRY.into()),
        ("+596", CountryCode::MTQ.into()),
        ("+597", CountryCode::SUR.into()),
        ("+598", CountryCode::URY.into()),
        ("+599", CountryCode::CUW.into()),
        ("+599", CountryCode::BES.into()), // Bonaire, Sint Eustatius and Saba
    ]
}

pub fn create_5_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+51", CountryCode::PER.into()),
        ("+52", CountryCode::MEX.into()),
        ("+53", CountryCode::CUB.into()),
        ("+54", CountryCode::ARG.into()),
        ("+55", CountryCode::BRA.into()),
        ("+56", CountryCode::CHL.into()),
        ("+57", CountryCode::COL.into()),
        ("+58", CountryCode::VEN.into()),
    ]
}

pub fn create_6_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+670", CountryCode::TLS.into()),
        ("+672", CountryCode::NFK.into()),
        ("+673", CountryCode::BRN.into()),
        ("+674", CountryCode::NRU.into()),
        ("+675", CountryCode::PNG.into()),
        ("+676", CountryCode::TON.into()),
        ("+677", CountryCode::SLB.into()),
        ("+678", CountryCode::VUT.into()),
        ("+679", CountryCode::FJI.into()),
        ("+680", CountryCode::PLW.into()),
        ("+681", CountryCode::WLF.into()),
        ("+682", CountryCode::COK.into()),
        ("+683", CountryCode::NIU.into()),
        ("+685", CountryCode::WSM.into()),
        ("+686", CountryCode::KIR.into()),
        ("+687", CountryCode::NCL.into()),
        ("+688", CountryCode::TUV.into()),
        ("+689", CountryCode::PYF.into()),
        ("+690", CountryCode::TKL.into()),
        ("+691", CountryCode::FSM.into()),
        ("+692", CountryCode::MHL.into()),
    ]
}

pub fn create_6_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+60", CountryCode::MYS.into()),
        ("+61", CountryCode::AUS.into()),
        ("+61", CountryCode::CXR.into()), // Christmas Island
        ("+61", CountryCode::CCK.into()), // Cocos Islands
        ("+62", CountryCode::IDN.into()),
        ("+63", CountryCode::PHL.into()),
        ("+64", CountryCode::NZL.into()),
        ("+64", CountryCode::PCN.into()), // Pitcairn Islands
        ("+65", CountryCode::SGP.into()),
        ("+66", CountryCode::THA.into()),
    ]
}

pub fn create_8_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+850", CountryCode::PRK.into()),
        ("+852", CountryCode::HKG.into()),
        ("+853", CountryCode::MAC.into()),
        ("+855", CountryCode::KHM.into()),
        ("+856", CountryCode::LAO.into()),
        ("+880", CountryCode::BGD.into()),
        ("+886", CountryCode::TWN.into()),
        ("+800", PhoneCountryCode::UniversalInternationalFreephone), // Universal International Freephone
        ("+808", PhoneCountryCode::InternationalSharedCost),         // International Shared Cost
        ("+870", PhoneCountryCode::InmarsatSNAC),                    // Inmarsat SNAC
        ("+881", PhoneCountryCode::GlobalMobileSatelliteSystem), // Global Mobile Satellite System
        ("+882", PhoneCountryCode::InternationalNetworks),       // International Networks
        ("+883", PhoneCountryCode::InternationalNetworks),       // International Networks
    ]
}

pub fn create_8_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+81", CountryCode::JPN.into()),
        ("+82", CountryCode::KOR.into()),
        ("+84", CountryCode::VNM.into()),
        ("+86", CountryCode::CHN.into()),
    ]
}

pub fn create_9_3() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+960", CountryCode::MDV.into()),
        ("+961", CountryCode::LBN.into()),
        ("+962", CountryCode::JOR.into()),
        ("+963", CountryCode::SYR.into()),
        ("+964", CountryCode::IRQ.into()),
        ("+965", CountryCode::KWT.into()),
        ("+966", CountryCode::SAU.into()),
        ("+967", CountryCode::YEM.into()),
        ("+968", CountryCode::OMN.into()),
        ("+970", CountryCode::PSE.into()),
        ("+971", CountryCode::ARE.into()),
        ("+972", CountryCode::ISR.into()),
        ("+973", CountryCode::BHR.into()),
        ("+974", CountryCode::QAT.into()),
        ("+975", CountryCode::BTN.into()),
        ("+976", CountryCode::MNG.into()),
        ("+977", CountryCode::NPL.into()),
        ("+992", CountryCode::TJK.into()),
        ("+993", CountryCode::TKM.into()),
        ("+994", CountryCode::AZE.into()),
        ("+995", CountryCode::GEO.into()),
        ("+996", CountryCode::KGZ.into()),
        ("+998", CountryCode::UZB.into()),
    ]
}

pub fn create_9_2() -> Vec<(&'static str, PhoneCountryCode)> {
    vec![
        ("+90", CountryCode::TUR.into()),
        ("+91", CountryCode::IND.into()),
        ("+92", CountryCode::PAK.into()),
        ("+93", CountryCode::AFG.into()),
        ("+94", CountryCode::LKA.into()),
        ("+95", CountryCode::MMR.into()),
        ("+98", CountryCode::IRN.into()),
    ]
}
