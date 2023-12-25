use core::fmt;

use log::error;

#[derive(Debug)]
pub enum ObisReference {
    VersionInformation,
    MessageDateTimeStamp,
    ElectricityEquipmentIdentifier,
    MeterReadingElectricityDeliveredToClientTariff1,
    MeterReadingElectricityDeliveredToClientTariff2,
    MeterReadingElectricityDeliveredByClientTariff1,
    MeterReadingElectricityDeliveredByClientTariff2,
    TariffIndicatorElectricity,
    ActualElectricityPowerDelivered,
    ActualElectricityPowerReceived,
    NumberPowerFailuresAnyPhase,
    NumberLongPowerFailuresAnyPhase,
    PowerFailureEventLog,
    NumberVoltageSagsPhaseL1,
    NumberVoltageSagsPhaseL2,
    NumberVoltageSagsPhaseL3,
    NumberVoltageSwellsPhaseL1,
    NumberVoltageSwellsPhaseL2,
    NumberVoltageSwellsPhaseL3,
    TextMessageCodes,
    TextMessageMaxCharacters,
    InstantaneousCurrentL1,
    InstantaneousCurrentL2,
    InstantaneousCurrentL3,
    InstantaneousActivePowerPositiveL1,
    InstantaneousActivePowerPositiveL2,
    InstantaneousActivePowerPositiveL3,
    InstantaneousActivePowerNegativeL1,
    InstantaneousActivePowerNegativeL2,
    InstantaneousActivePowerNegativeL3,
    DeviceType,
    OtherEquipementIdentifier,
    LastHourlyValueMeterReading,
}

impl ObisReference {
    pub fn from_message(message: &str) -> Option<Self> {
        let code = message.split('(').next();

        if code.is_none() {
            error!(
                "OBIS reference uses an unexpected format because no '(' was found: {}",
                message
            );
            return None;
        }

        // All OBIS references with variable decimal places start with "0-n:", where "n" is a placeholder for the number of decimals
        // This means that we can safely match on the values after the colon without taking the start of the code into account
        match &code.unwrap()[4..] {
            "0.2.8" => Some(Self::VersionInformation),
            "1.0.0" => Some(Self::MessageDateTimeStamp),
            "96.1.1" => Some(Self::ElectricityEquipmentIdentifier),
            "1.8.1" => Some(Self::MeterReadingElectricityDeliveredToClientTariff1),
            "1.8.2" => Some(Self::MeterReadingElectricityDeliveredToClientTariff2),
            "2.8.1" => Some(Self::MeterReadingElectricityDeliveredByClientTariff1),
            "2.8.2" => Some(Self::MeterReadingElectricityDeliveredByClientTariff2),
            "96.14.0" => Some(Self::TariffIndicatorElectricity),
            "1.7.0" => Some(Self::ActualElectricityPowerDelivered),
            "2.7.0" => Some(Self::ActualElectricityPowerReceived),
            "96.7.21" => Some(Self::NumberPowerFailuresAnyPhase),
            "96.7.9" => Some(Self::NumberLongPowerFailuresAnyPhase),
            "99.97.0" => Some(Self::PowerFailureEventLog),
            "32.32.0" => Some(Self::NumberVoltageSagsPhaseL1),
            "52.32.0" => Some(Self::NumberVoltageSagsPhaseL2),
            "72:32.0" => Some(Self::NumberVoltageSagsPhaseL3),
            "32.36.0" => Some(Self::NumberVoltageSwellsPhaseL1),
            "52.36.0" => Some(Self::NumberVoltageSwellsPhaseL2),
            "72.36.0" => Some(Self::NumberVoltageSwellsPhaseL3),
            "96.13.1" => Some(Self::TextMessageCodes),
            "96.13.0" => Some(Self::TextMessageMaxCharacters),
            "31.7.0" => Some(Self::InstantaneousCurrentL1),
            "51.7.0" => Some(Self::InstantaneousCurrentL2),
            "71.7.0" => Some(Self::InstantaneousCurrentL3),
            "21.7.0" => Some(Self::InstantaneousActivePowerPositiveL1),
            "41.7.0" => Some(Self::InstantaneousActivePowerPositiveL2),
            "61.7.0" => Some(Self::InstantaneousActivePowerPositiveL3),
            "22.7.0" => Some(Self::InstantaneousActivePowerNegativeL1),
            "42.7.0" => Some(Self::InstantaneousActivePowerNegativeL2),
            "62.7.0" => Some(Self::InstantaneousActivePowerNegativeL3),
            "24.1.0" => Some(Self::DeviceType),
            "96.1.0" => Some(Self::OtherEquipementIdentifier),
            "24.2.1" => Some(Self::LastHourlyValueMeterReading),
            _ => None,
        }
    }

    pub fn get_value(message: &str) -> String {
        let value_start = message.find('(').unwrap() + 1;
        let value_end = message.find(')').unwrap();
        message[value_start..value_end].to_owned()
    }
}

impl fmt::Display for ObisReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
