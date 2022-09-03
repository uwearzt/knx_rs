// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------
// / See KNX Standard document 03_07_02-Datapoint-Types-v02.02.01-AS.pdf for a 
// / definition of all DataPoint Types.

use knx_macro::dpt_types;

#[derive(PartialEq, Debug)]
#[derive(dpt_types)]
#[allow(non_camel_case_types)]
pub enum DPT {
     #[dpt_type(main = 1, sub = 1)]
    DPT_Switch,
    #[dpt_type(main = 1, sub = 2)]
    DPT_Bool,
    #[dpt_type(main = 1, sub = 3)]
    DPT_Enable,
    #[dpt_type(main = 1, sub = 4)]
    DPT_Ramp,
    #[dpt_type(main = 1, sub = 5)]
    DPT_Alarm,
    #[dpt_type(main = 1, sub = 6)]
    DPT_BinaryValue,
    #[dpt_type(main = 1, sub = 7)]
    DPT_Step,
    #[dpt_type(main = 1, sub = 8)]
    DPT_UpDown,
    #[dpt_type(main = 1, sub = 9)]
    DPT_OpenClose,
    #[dpt_type(main = 1, sub = 10)]
    DPT_Start,
    #[dpt_type(main = 1, sub = 11)]
    DPT_State,
    #[dpt_type(main = 1, sub = 12)]
    DPT_Invert,
    #[dpt_type(main = 1, sub = 13)]
    DPT_DimSendStyle,
    #[dpt_type(main = 1, sub = 14)]
    DPT_InputSource,
    #[dpt_type(main = 1, sub = 15)]
    DPT_Reset,
    #[dpt_type(main = 1, sub = 16)]
    DPT_Ack,
    #[dpt_type(main = 1, sub = 17)]
    DPT_Trigger,
    #[dpt_type(main = 1, sub = 18)]
    DPT_Occupancy,
    #[dpt_type(main = 1, sub = 19)]
    DPT_Window_Door,
    #[dpt_type(main = 1, sub = 21)]
    DPT_LogicalFunction,
    #[dpt_type(main = 1, sub = 22)]
    DPT_Scene_AB,
    #[dpt_type(main = 1, sub = 23)]
    DPT_ShutterBlinds_Mode,
    #[dpt_type(main = 1, sub = 24)]
    DPT_DayNight,
    #[dpt_type(main = 1, sub = 100)]
    DPT_Heat_Cool,
    #[dpt_type(main = 11, sub = 200)]
    DPT_ConsumerProducer,
    #[dpt_type(main = 11, sub = 201)]
    DPT_EnergyDirection,
    #[dpt_type(main = 2, sub = 1)]
    DPT_Switch_Control,
    #[dpt_type(main = 2, sub = 2)]
    DPT_Bool_Control,
    #[dpt_type(main = 2, sub = 3)]
    DPT_Enable_Control,
    #[dpt_type(main = 2, sub = 4)]
    DPT_Ramp_Control,
    #[dpt_type(main = 2, sub = 5)]
    DPT_Alarm_Control,
    #[dpt_type(main = 2, sub = 6)]
    DPT_BinaryValue_Control,
    #[dpt_type(main = 2, sub = 7)]
    DPT_Step_Control,
    #[dpt_type(main = 2, sub = 8)]
    DPT_Direction_1_Control,
    #[dpt_type(main = 2, sub = 9)]
    DPT_Direction2_Control,
    #[dpt_type(main = 2, sub = 10)]
    DPT_Start_Control,
    #[dpt_type(main = 2, sub = 11)]
    DPT_State_Control,
    #[dpt_type(main = 2, sub = 12)]
    DPT_Invert_Control,
    #[dpt_type(main = 3, sub = 7)]
    DPT_Control_Dimming,
    #[dpt_type(main = 3, sub = 8)]
    DPT_Control_Blinds,
    #[dpt_type(main = 4, sub = 1)]
    DPT_Char_ASCII,
    #[dpt_type(main = 4, sub = 2)]
    DPT_Char_8859_1,
    #[dpt_type(main = 5, sub = 1)]
    DPT_Scaling,
    #[dpt_type(main = 5, sub = 3)]
    DPT_Angle,
    #[dpt_type(main = 5, sub = 4)]
    DPT_Percent_U8,
    #[dpt_type(main = 5, sub = 5)]
    DPT_DecimalFactor,
    #[dpt_type(main = 5, sub = 6)]
    DPT_Tariff,
    #[dpt_type(main = 5, sub = 10)]
    DPT_Value_1_Ucount,
    #[dpt_type(main = 6, sub = 1)]
    DPT_Percent_V8,
    #[dpt_type(main = 6, sub = 10)]
    DPT_Value_1_Count,
    #[dpt_type(main = 6, sub = 20)]
    DPT_Status_Mode3,
    #[dpt_type(main = 7, sub = 1)]
    DPT_Value_2_Ucount,
    #[dpt_type(main = 7, sub = 2)]
    DPT_TimePeriodMsec,
    #[dpt_type(main = 7, sub = 3)]
    DPT_TimePeriodlOMSec,
    #[dpt_type(main = 7, sub = 4)]
    DPT_TimePeriodlOOMSec,
    #[dpt_type(main = 7, sub = 5)]
    DPT_TimePeriodSec,
    #[dpt_type(main = 7, sub = 6)]
    DPT_TimePeriodMin,
    #[dpt_type(main = 7, sub = 7)]
    DPT_TimePeriodHrs,
    #[dpt_type(main = 7, sub = 10)]
    DPT_PropDataType,
    #[dpt_type(main = 7, sub = 11)]
    DPT_Length_mm,
    #[dpt_type(main = 7, sub = 12)]
    DPT_UEICurrentmA,
    #[dpt_type(main = 7, sub = 13)]
    DPT_Brightness,
    #[dpt_type(main = 7, sub = 600)]
    DPT_Absolute_Colour_Temperature,
    #[dpt_type(main = 8, sub = 1)]
    DPT_Value_2_Count,
    #[dpt_type(main = 8, sub = 2)]
    DPT_DeltaTimeMsec,
    #[dpt_type(main = 8, sub = 3)]
    DPT_DeltaTimelOMSec,
    #[dpt_type(main = 8, sub = 4)]
    DPT_DeltaTimelOOMSec,
    #[dpt_type(main = 8, sub = 5)]
    DPT_DeltaTimeSec,
    #[dpt_type(main = 8, sub = 6)]
    DPT_DeltaTimeMin,
    #[dpt_type(main = 8, sub = 7)]
    DPT_DeltaTimeHrs,
    #[dpt_type(main = 8, sub = 10)]
    DPT_Percent_Vi6,
    #[dpt_type(main = 8, sub = 11)]
    DPT_Rotation_Angle,
    #[dpt_type(main = 8, sub = 12)]
    DPT_Length_m,
    #[dpt_type(main = 9, sub = 1)]
    DPT_Value_Temp,
    #[dpt_type(main = 9, sub = 2)]
    DPT_Value_Tempd,
    #[dpt_type(main = 9, sub = 3)]
    DPT_Value_Tempa,
    #[dpt_type(main = 9, sub = 4)]
    DPT_Value_Lux,
    #[dpt_type(main = 9, sub = 5)]
    DPT_Value_Wsp,
    #[dpt_type(main = 9, sub = 6)]
    DPT_Value_Pres,
    #[dpt_type(main = 9, sub = 7)]
    DPT_Value_Humidity,
    #[dpt_type(main = 9, sub = 8)]
    DPT_Value_AirQuality,
    #[dpt_type(main = 9, sub = 9)]
    DPT_Value_AirFlow,
    #[dpt_type(main = 9, sub = 10)]
    DPT_Value_Timel,
    #[dpt_type(main = 9, sub = 11)]
    DPT_Value_Time2,
    #[dpt_type(main = 9, sub = 20)]
    DPT_Value_Volt,
    #[dpt_type(main = 9, sub = 21)]
    DPT_Value_Curr,
    #[dpt_type(main = 9, sub = 22)]
    DPT_PowerDensity,
    #[dpt_type(main = 9, sub = 23)]
    DPT_KelvinPerPercent,
    #[dpt_type(main = 9, sub = 24)]
    DPT_Power,
    #[dpt_type(main = 9, sub = 25)]
    DPT_Value_Volume_Flow,
    #[dpt_type(main = 9, sub = 26)]
    DPT_Rain_Amount,
    #[dpt_type(main = 9, sub = 27)]
    DPT_Value_Temp_F,
    #[dpt_type(main = 9, sub = 28)]
    DPT_Value_Wsp_kmh,
    #[dpt_type(main = 9, sub = 29)]
    DPT_Value_Absolute_Humidity,
    #[dpt_type(main = 9, sub = 30)]
    DPT_Concentration_pgm3,
    #[dpt_type(main = 10, sub = 1)]
    DPT_TimeOfDay,
    #[dpt_type(main = 11, sub = 1)]
    DPT_Date,
    #[dpt_type(main = 12, sub = 1)]
    DPT_Value_4_Ucount,
    #[dpt_type(main = 12, sub = 100)]
    DPT_LongTimePeriod_Sec,
    #[dpt_type(main = 12, sub = 101)]
    DPT_LongTimePeriod_Min,
    #[dpt_type(main = 12, sub = 102)]
    DPT_LongTimePeriod_Hrs,
    #[dpt_type(main = 121, sub = 200)]
    DPT_VolumeLiguid_Litre,
    #[dpt_type(main = 121, sub = 201)]
    DPT_Volume_m3,
    #[dpt_type(main = 13, sub = 1)]
    DPT_Value_4_Count,
    #[dpt_type(main = 13, sub = 2)]
    DPT_FlowRate_m3h,
    #[dpt_type(main = 13, sub = 10)]
    DPT_ActiveEnergy,
    #[dpt_type(main = 13, sub = 11)]
    DPT_ApparantEnergy,
    #[dpt_type(main = 13, sub = 12)]
    DPT_ReactiveEnergy,
    #[dpt_type(main = 13, sub = 13)]
    DPT_ActiveEnergy_kWh,
    #[dpt_type(main = 13, sub = 14)]
    DPT_ApparantEnergy_kVAh,
    #[dpt_type(main = 13, sub = 15)]
    DPT_ReactiveEnergy_kVARh,
    #[dpt_type(main = 13, sub = 16)]
    DPT_ActiveEnergy_MWh,
    #[dpt_type(main = 13, sub = 100)]
    DPT_LongDeltaTimeSec,
    #[dpt_type(main = 131, sub = 200)]
    DPT_DeltaVolumeLiguid_Litre,
    #[dpt_type(main = 131, sub = 201)]
    DPT_DeltaVolume_m3,
    #[dpt_type(main = 14, sub = 0)]
    DPT_Value_Acceleration,
    #[dpt_type(main = 14, sub = 1)]
    DPT_Value_Acceleration_Angular,
    #[dpt_type(main = 14, sub = 2)]
    DPT_Value_Activation_Energy,
    #[dpt_type(main = 14, sub = 3)]
    DPT_Value_Activity,
    #[dpt_type(main = 14, sub = 4)]
    DPT_Value_Mol,
    #[dpt_type(main = 14, sub = 5)]
    DPT_Value_Amplitude,
    #[dpt_type(main = 14, sub = 6)]
    DPT_Value_AngleRad,
    #[dpt_type(main = 14, sub = 7)]
    DPT_Value_AngleDeg,
    #[dpt_type(main = 14, sub = 8)]
    DPT_Value_Angular_Momentum,
    #[dpt_type(main = 14, sub = 9)]
    DPT_Value_Angular_Velocity,
    #[dpt_type(main = 14, sub = 10)]
    DPT_Value_Area,
    #[dpt_type(main = 14, sub = 11)]
    DPT_Value_Capacitance,
    #[dpt_type(main = 14, sub = 12)]
    DPT_Value_Charge_DensitySurface,
    #[dpt_type(main = 14, sub = 13)]
    DPT_Value_Charge_DensityVolume,
    #[dpt_type(main = 14, sub = 14)]
    DPT_Value_Compressibility,
    #[dpt_type(main = 14, sub = 15)]
    DPT_Value_Conductance,
    #[dpt_type(main = 14, sub = 16)]
    DPT_Value_Electrical_Conductivity,
    #[dpt_type(main = 14, sub = 17)]
    DPT_Value_Density,
    #[dpt_type(main = 14, sub = 18)]
    DPT_Value_Electric_Charge,
    #[dpt_type(main = 14, sub = 19)]
    DPT_Value_Electric_Current,
    #[dpt_type(main = 14, sub = 20)]
    DPT_Value_Electric_CurrentDensity,
    #[dpt_type(main = 14, sub = 21)]
    DPT_Value_Electric_DipoleMoment,
    #[dpt_type(main = 14, sub = 22)]
    DPT_Value_Electric_Displacement,
    #[dpt_type(main = 14, sub = 23)]
    DPT_Value_Electric_FieldStrength,
    #[dpt_type(main = 14, sub = 24)]
    DPT_Value_Electric_Flux,
    #[dpt_type(main = 14, sub = 25)]
    DPT_Value_Electric_FluxDensity,
    #[dpt_type(main = 14, sub = 26)]
    DPT_Value_Electric_Polarization,
    #[dpt_type(main = 14, sub = 27)]
    DPT_Value_Electric_Potential,
    #[dpt_type(main = 14, sub = 28)]
    DPT_Value_Electric_PotentialDifference,
    #[dpt_type(main = 14, sub = 29)]
    DPT_Value_ElectromagneticMoment,
    #[dpt_type(main = 14, sub = 30)]
    DPT_Value_Electromotive_Force,
    #[dpt_type(main = 14, sub = 31)]
    DPT_Value_Energy,
    #[dpt_type(main = 14, sub = 32)]
    DPT_Value_Force,
    #[dpt_type(main = 14, sub = 33)]
    DPT_Value_Freguency,
    #[dpt_type(main = 14, sub = 34)]
    DPT_Value_Angular_Freguency,
    #[dpt_type(main = 14, sub = 35)]
    DPT_Value_Heat_Capacity,
    #[dpt_type(main = 14, sub = 36)]
    DPT_Value_Heat_FlowRate,
    #[dpt_type(main = 14, sub = 37)]
    DPT_Value_Heat_Quantity,
    #[dpt_type(main = 14, sub = 38)]
    DPT_Value_Impedance,
    #[dpt_type(main = 14, sub = 39)]
    DPT_Value_Length,
    #[dpt_type(main = 14, sub = 40)]
    DPT_Value_Light_Quantity,
    #[dpt_type(main = 14, sub = 41)]
    DPT_Value_Luminance,
    #[dpt_type(main = 14, sub = 42)]
    DPT_Value_Luminous_Flux,
    #[dpt_type(main = 14, sub = 43)]
    DPT_Value_Luminous_Intensity,
    #[dpt_type(main = 14, sub = 44)]
    DPT_Value_Magnetic_FieldStrength,
    #[dpt_type(main = 14, sub = 45)]
    DPT_Value_Magnetic_Flux,
    #[dpt_type(main = 14, sub = 46)]
    DPT_Value_Magnetic_FluxDensity,
    #[dpt_type(main = 14, sub = 47)]
    DPT_Value_Magnetic_Moment,
    #[dpt_type(main = 14, sub = 48)]
    DPT_Value_Magnetic_Polarization,
    #[dpt_type(main = 14, sub = 49)]
    DPT_Value_Magnetization,
    #[dpt_type(main = 14, sub = 50)]
    DPT_Value_MagnetomotiveForce,
    #[dpt_type(main = 14, sub = 51)]
    DPT_Value_Mass,
    #[dpt_type(main = 14, sub = 52)]
    DPT_Value_MassFlux,
    #[dpt_type(main = 14, sub = 53)]
    DPT_Value_Momentum,
    #[dpt_type(main = 14, sub = 54)]
    DPT_Value_Phase_AngleRad,
    #[dpt_type(main = 14, sub = 55)]
    DPT_Value_Phase_AngleDeg,
    #[dpt_type(main = 14, sub = 56)]
    DPT_Value_Power,
    #[dpt_type(main = 14, sub = 57)]
    DPT_Value_Power_Factor,
    #[dpt_type(main = 14, sub = 58)]
    DPT_Value_Pressure,
    #[dpt_type(main = 14, sub = 59)]
    DPT_Value_Reactance,
    #[dpt_type(main = 14, sub = 60)]
    DPT_Value_Resistance,
    #[dpt_type(main = 14, sub = 61)]
    DPT_Value_Resistivity,
    #[dpt_type(main = 14, sub = 62)]
    DPT_Value_Selfinductance,
    #[dpt_type(main = 14, sub = 63)]
    DPT_Value_SolidAngle,
    #[dpt_type(main = 14, sub = 64)]
    DPT_Value_Sound_Intensity,
    #[dpt_type(main = 14, sub = 65)]
    DPT_Value_Speed,
    #[dpt_type(main = 14, sub = 66)]
    DPT_Value_Stress,
    #[dpt_type(main = 14, sub = 67)]
    DPT_Value_Surface_Tension,
    #[dpt_type(main = 14, sub = 68)]
    DPT_Value_Common_Temperature,
    #[dpt_type(main = 14, sub = 69)]
    DPT_Value_Absolute_Temperature,
    #[dpt_type(main = 14, sub = 70)]
    DPT_Value_TemperatureDifference,
    #[dpt_type(main = 14, sub = 71)]
    DPT_Value_Thermal_Capacity,
    #[dpt_type(main = 14, sub = 72)]
    DPT_Value_Thermal_Conductivity,
    #[dpt_type(main = 14, sub = 73)]
    DPT_Value_ThermoelectricPower,
    #[dpt_type(main = 14, sub = 74)]
    DPT_Value_Time,
    #[dpt_type(main = 14, sub = 75)]
    DPT_Value_Torgue,
    #[dpt_type(main = 14, sub = 76)]
    DPT_Value_Volume,
    #[dpt_type(main = 14, sub = 77)]
    DPT_Value_Volume_Flux,
    #[dpt_type(main = 14, sub = 78)]
    DPT_Value_Weight,
    #[dpt_type(main = 14, sub = 79)]
    DPT_Value_Work,
    #[dpt_type(main = 14, sub = 80)]
    DPT_Value_ApparentPower,
    #[dpt_type(main = 141, sub = 200)]
    DPT_Volume_Flux_Meter,
    #[dpt_type(main = 141, sub = 201)]
    DPT_Volume_Flux_Is,
    #[dpt_type(main = 15, sub = 0)]
    DPT_Access_Data,
    #[dpt_type(main = 16, sub = 0)]
    DPT_String_ASCII,
    #[dpt_type(main = 16, sub = 1)]
    DPT_String_8859_1,
    #[dpt_type(main = 17, sub = 1)]
    DPT_SceneNumber,
    #[dpt_type(main = 18, sub = 1)]
    DPT_SceneControl,
    #[dpt_type(main = 19, sub = 1)]
    DPT_DateTime,
    #[dpt_type(main = 20, sub = 1)]
    DPT_SCLOMode,
    #[dpt_type(main = 20, sub = 2)]
    DPT_BuildingMode,
    #[dpt_type(main = 20, sub = 3)]
    DPT_OccMode,
    #[dpt_type(main = 20, sub = 4)]
    DPT_Priority,
    #[dpt_type(main = 20, sub = 5)]
    DPT_LightApplicationMode,
    #[dpt_type(main = 20, sub = 6)]
    DPT_ApplicationArea,
    #[dpt_type(main = 20, sub = 7)]
    DPT_AlarmClassType,
    #[dpt_type(main = 20, sub = 8)]
    DPT_PSUMode,
    #[dpt_type(main = 20, sub = 11)]
    DPT_ErrorClass_System,
    #[dpt_type(main = 20, sub = 12)]
    DPT_ErrorClass_HVAC,
    #[dpt_type(main = 20, sub = 13)]
    DPT_Time_Delay,
    #[dpt_type(main = 20, sub = 14)]
    DPT_Beaufort_Wind_Force_Scale,
    #[dpt_type(main = 20, sub = 17)]
    DPT_SensorSelect,
    #[dpt_type(main = 20, sub = 20)]
    DPT_ActuatorConnectType,
    #[dpt_type(main = 20, sub = 21)]
    DPT_Cloud_Cover,
    #[dpt_type(main = 20, sub = 22)]
    DPT_PowerReturnMode,
    #[dpt_type(main = 20, sub = 100)]
    DPT_FuelType,
    #[dpt_type(main = 20, sub = 101)]
    DPT_BurnerType,
    #[dpt_type(main = 20, sub = 102)]
    DPT_HVACMode,
    #[dpt_type(main = 20, sub = 103)]
    DPT_DHWMode,
    #[dpt_type(main = 20, sub = 104)]
    DPT_LoadPriority,
    #[dpt_type(main = 20, sub = 105)]
    DPT_HVACContrMode,
    #[dpt_type(main = 20, sub = 106)]
    DPT_HVACEmergMode,
    #[dpt_type(main = 20, sub = 107)]
    DPT_ChangeoverMode,
    #[dpt_type(main = 20, sub = 108)]
    DPT_ValveMode,
    #[dpt_type(main = 20, sub = 109)]
    DPT_DamperMode,
    #[dpt_type(main = 20, sub = 110)]
    DPT_HeaterMode,
    #[dpt_type(main = 20, sub = 111)]
    DPT_FanMode,
    #[dpt_type(main = 20, sub = 112)]
    DPT_MasterSlaveMode,
    #[dpt_type(main = 20, sub = 113)]
    DPT_StatusRoomSetp,
    #[dpt_type(main = 20, sub = 114)]
    DPT_Metering_DeviceType,
    #[dpt_type(main = 20, sub = 115)]
    DPT_HumDehumMode,
    #[dpt_type(main = 20, sub = 120)]
    DPT_ADAType,
    #[dpt_type(main = 20, sub = 121)]
    DPT_BackupMode,
    #[dpt_type(main = 20, sub = 122)]
    DPT_StartSynchronization,
    #[dpt_type(main = 20, sub = 600)]
    DPT_Behaviour_Lock_Unlock,
    #[dpt_type(main = 20, sub = 601)]
    DPT_Behaviour_Bus_Power_Up_Down,
    #[dpt_type(main = 20, sub = 602)]
    DPT_DALI_Fade_Time,
    #[dpt_type(main = 20, sub = 603)]
    DPT_BlinkingMode,
    #[dpt_type(main = 20, sub = 604)]
    DPT_LightControlMode,
    #[dpt_type(main = 20, sub = 605)]
    DPT_SwitchPBModel,
    #[dpt_type(main = 20, sub = 606)]
    DPT_PBAction,
    #[dpt_type(main = 20, sub = 607)]
    DPT_DimmPBModel,
    #[dpt_type(main = 20, sub = 608)]
    DPT_SwitchOnMode,
    #[dpt_type(main = 20, sub = 609)]
    DPT_LoadTypeSet,
    #[dpt_type(main = 20, sub = 610)]
    DPT_LoadTypeDetected,
    #[dpt_type(main = 20, sub = 611)]
    DPT_Converter_Test_Control,
    #[dpt_type(main = 20, sub = 612)]
    DPT_Converter_Control,
    #[dpt_type(main = 20, sub = 613)]
    DPT_Converter_Data_Reguest,
    #[dpt_type(main = 20, sub = 801)]
    DPT_SABExceptBehaviour,
    #[dpt_type(main = 20, sub = 802)]
    DPT_SABBehaviour_Lock_Unlock,
    #[dpt_type(main = 20, sub = 803)]
    DPT_SSSBMode,
    #[dpt_type(main = 20, sub = 804)]
    DPT_BlindsControlMode,
    #[dpt_type(main = 201, sub = 0)]
    DPT_CommMode,
    #[dpt_type(main = 201, sub = 1)]
    DPT_AddlnfoTypes,
    #[dpt_type(main = 201, sub = 2)]
    DPT_RF_ModeSelect,
    #[dpt_type(main = 201, sub = 3)]
    DPT_RF_FilterSelect,
    #[dpt_type(main = 201, sub = 4)]
    DPT_Medium,
    #[dpt_type(main = 201, sub = 5)]
    DPT_PB_Function,
    #[dpt_type(main = 201, sub = 200)]
    DPT_MBus_BreakerValve_State,
    #[dpt_type(main = 201, sub = 202)]
    DPT_Gas_Measurement_Condition,
    #[dpt_type(main = 201, sub = 203)]
    DPT_Breaker_Status,
    #[dpt_type(main = 201, sub = 204)]
    DPT_Euridis_Communication_Interface_Status,
    #[dpt_type(main = 201, sub = 205)]
    DPT_PLC_Status,
    #[dpt_type(main = 201, sub = 206)]
    DPT_Peak_Event_Notice,
    #[dpt_type(main = 201, sub = 207)]
    DPT_Peak_Event,
    #[dpt_type(main = 201, sub = 208)]
    DPT_TIC_Type,
    #[dpt_type(main = 201, sub = 209)]
    DPT_Type_TIC_Channel,
    #[dpt_type(main = 21, sub = 1)]
    DPT_StatusGen,
    #[dpt_type(main = 21, sub = 2)]
    DPT_Device_Control,
    #[dpt_type(main = 21, sub = 100)]
    DPT_ForceSign,
    #[dpt_type(main = 21, sub = 101)]
    DPT_ForceSignCool,
    #[dpt_type(main = 21, sub = 102)]
    DPT_StatusRHC,
    #[dpt_type(main = 21, sub = 103)]
    DPT_StatusSDHWC,
    #[dpt_type(main = 21, sub = 104)]
    DPT_FuelTypeSet,
    #[dpt_type(main = 21, sub = 105)]
    DPT_StatusRCC,
    #[dpt_type(main = 21, sub = 106)]
    DPT_StatusAHU,
    #[dpt_type(main = 21, sub = 601)]
    DPT_LightActuatorErrorlnfo,
    #[dpt_type(main = 211, sub = 0)]
    DPT_RF_Modelnfo,
    #[dpt_type(main = 211, sub = 1)]
    DPT_RF_Filterlnfo,
    #[dpt_type(main = 211, sub = 2)]
    DPT_Security_Report,
    #[dpt_type(main = 211, sub = 10)]
    DPT_Channel_Activation_8,
    #[dpt_type(main = 211, sub = 200)]
    DPT_VirtualDryContact,
    #[dpt_type(main = 211, sub = 201)]
    DPT_Phases_Status,
    #[dpt_type(main = 22, sub = 100)]
    DPT_StatusDHWC,
    #[dpt_type(main = 22, sub = 101)]
    DPT_StatusRHCC,
    #[dpt_type(main = 221, sub = 0)]
    DPT_Media,
    #[dpt_type(main = 221, sub = 10)]
    DPT_Channel_Activation_16,
    #[dpt_type(main = 23, sub = 1)]
    DPT_OnOff_Action,
    #[dpt_type(main = 23, sub = 2)]
    DPT_Alarm_Reaction,
    #[dpt_type(main = 23, sub = 3)]
    DPT_UpDown_Action,
    #[dpt_type(main = 23, sub = 102)]
    DPT_HVAC_PB_Action,
    #[dpt_type(main = 24, sub = 1)]
    DPT_VarString_8859_1,
    #[dpt_type(main = 251, sub = 0)]
    DPT_DoubleNibble,
    #[dpt_type(main = 26, sub = 1)]
    DPT_Scenelnfo,
    #[dpt_type(main = 27, sub = 1)]
    DPT_CombinedInfoOnOff,
    #[dpt_type(main = 28, sub = 1)]
    DPT_UTF8,
    #[dpt_type(main = 29, sub = 10)]
    DPT_ActiveEnergy_V64,
    #[dpt_type(main = 29, sub = 11)]
    DPT_ApparantEnergy_V64,
    #[dpt_type(main = 29, sub = 12)]
    DPT_ReactiveEnergy_V64,
    #[dpt_type(main = 301, sub = 10)]
    DPT_Channel_Activation_24,
    #[dpt_type(main = 31, sub = 101)]
    DPT_PB_Action_HVAC_Extended,
    #[dpt_type(main = 200, sub = 100)]
    DPT_Heat_Cool_Z,
    #[dpt_type(main = 200, sub = 101)]
    DPT_BinaryValue_Z,
    #[dpt_type(main = 201, sub = 100)]
    DPT_HVACMode_Z,
    #[dpt_type(main = 201, sub = 102)]
    DPT_DHWMode_Z,
    #[dpt_type(main = 201, sub = 104)]
    DPT_HVACContrMode_Z,
    #[dpt_type(main = 201, sub = 105)]
    DPT_EnabIH_Cstage_Z,
    #[dpt_type(main = 201, sub = 107)]
    DPT_BuildingMode_Z,
    #[dpt_type(main = 201, sub = 108)]
    DPT_OccMode_Z,
    #[dpt_type(main = 201, sub = 109)]
    DPT_HVACEmergMode_Z,
    #[dpt_type(main = 202, sub = 1)]
    DPT_RelValue_Z,
    #[dpt_type(main = 202, sub = 2)]
    DPT_UCountValue8_Z,
    #[dpt_type(main = 203, sub = 2)]
    DPT_TimePeriodMsec_Z,
    #[dpt_type(main = 203, sub = 3)]
    DPT_TimePeriodlOMsec_Z,
    #[dpt_type(main = 203, sub = 4)]
    DPT_TimePeriodlOOMsec_Z,
    #[dpt_type(main = 203, sub = 5)]
    DPT_TimePeriodSec_Z,
    #[dpt_type(main = 203, sub = 6)]
    DPT_TimePeriodMin_Z,
    #[dpt_type(main = 203, sub = 7)]
    DPT_TimePeriodHrs_Z,
    #[dpt_type(main = 203, sub = 11)]
    DPT_UFIowRateLiter_h_Z,
    #[dpt_type(main = 203, sub = 12)]
    DPT_UCountValue16_Z,
    #[dpt_type(main = 203, sub = 13)]
    DPT_UEICurrent_uA_Z,
    #[dpt_type(main = 203, sub = 14)]
    DPT_PowerKW_Z,
    #[dpt_type(main = 203, sub = 15)]
    DPT_Atm_PressureAbs_Z,
    #[dpt_type(main = 203, sub = 17)]
    DPT_PercentU16_Z,
    #[dpt_type(main = 203, sub = 100)]
    DPT_HVACAirQual_Z,
    #[dpt_type(main = 203, sub = 101)]
    DPT_WindSpeed_Z,
    #[dpt_type(main = 203, sub = 102)]
    DPT_Sunlntensity_Z,
    #[dpt_type(main = 203, sub = 104)]
    DPT_HVACAirFlowAbs_Z,
    #[dpt_type(main = 204, sub = 1)]
    DPT_RelSignedValue_Z,
    #[dpt_type(main = 205, sub = 2)]
    DPT_DeltaTimeMsec_Z,
    #[dpt_type(main = 205, sub = 3)]
    DPT_DeltaTimelOMsec_Z,
    #[dpt_type(main = 205, sub = 4)]
    DPT_DeltaTimelOOMsec_Z,
    #[dpt_type(main = 205, sub = 5)]
    DPT_DeltaTimeSec_Z,
    #[dpt_type(main = 205, sub = 6)]
    DPT_DeltaTimeMin_Z,
    #[dpt_type(main = 205, sub = 7)]
    DPT_DeltaTimeHrs_Z,
    #[dpt_type(main = 205, sub = 17)]
    DPT_Percent_VI6_Z,
    #[dpt_type(main = 205, sub = 100)]
    DPT_TempHVACAbs_Z,
    #[dpt_type(main = 205, sub = 101)]
    DPT_TempHVACRel_Z,
    #[dpt_type(main = 205, sub = 102)]
    DPT_HVACAirFlowRel_Z,
    #[dpt_type(main = 205, sub = 103)]
    DPT_HVACAirQualiRel_Z,
    #[dpt_type(main = 206, sub = 100)]
    DPT_HVACModeNext,
    #[dpt_type(main = 206, sub = 102)]
    DPT_DHWModeNext,
    #[dpt_type(main = 206, sub = 104)]
    DPT_OccModeNext,
    #[dpt_type(main = 206, sub = 105)]
    DPT_BuildingModeNext,
    #[dpt_type(main = 207, sub = 100)]
    DPT_StatusBUC,
    #[dpt_type(main = 207, sub = 101)]
    DPT_LockSign,
    #[dpt_type(main = 207, sub = 102)]
    DPT_ValueDemBOC,
    #[dpt_type(main = 207, sub = 104)]
    DPT_ActPosDemAbs,
    #[dpt_type(main = 207, sub = 105)]
    DPT_StatusAct,
    #[dpt_type(main = 207, sub = 600)]
    DPT_StatusLightingActuator,
    #[dpt_type(main = 209, sub = 100)]
    DPT_StatusHPM,
    #[dpt_type(main = 209, sub = 101)]
    DPT_TempRoomDemAbs,
    #[dpt_type(main = 209, sub = 102)]
    DPT_StatusCPM,
    #[dpt_type(main = 209, sub = 103)]
    DPT_StatusWTC,
    #[dpt_type(main = 210, sub = 100)]
    DPT_TempFlowWaterDemAbs,
    #[dpt_type(main = 211, sub = 100)]
    DPT_EnergyDemWater,
    #[dpt_type(main = 212, sub = 100)]
    DPT_TempRoomSetpSetShiftr31,
    #[dpt_type(main = 212, sub = 101)]
    DPT_TempRoomSetpSet_3,
    #[dpt_type(main = 213, sub = 100)]
    DPT_TempRoomSetpSet_4,
    #[dpt_type(main = 213, sub = 101)]
    DPT_TempDHWSetpSetKl,
    #[dpt_type(main = 213, sub = 102)]
    DPT_TempRoomSetpSetShift_4,
    #[dpt_type(main = 214, sub = 100)]
    DPT_PowerFlowWaterDemHPM,
    #[dpt_type(main = 214, sub = 101)]
    DPT_PowerFlowWaterDemCPM,
    #[dpt_type(main = 215, sub = 100)]
    DPT_StatusBOC,
    #[dpt_type(main = 215, sub = 101)]
    DPT_StatusCC,
    #[dpt_type(main = 216, sub = 100)]
    DPT_SpecHeatProd,
    #[dpt_type(main = 217, sub = 1)]
    DPT_Version,
    #[dpt_type(main = 218, sub = 1)]
    DPT_VolumeLiter_Z,
    #[dpt_type(main = 218, sub = 2)]
    DPT_FlowRate_m3h_Z,
    #[dpt_type(main = 219, sub = 1)]
    DPT_Alarmlnfo,
    #[dpt_type(main = 220, sub = 100)]
    DPT_TempHVACAbsNext,
    #[dpt_type(main = 221, sub = 1)]
    DPT_SerNum,
    #[dpt_type(main = 222, sub = 100)]
    DPT_TempRoomSetpSetF16_3,
    #[dpt_type(main = 222, sub = 101)]
    DPT_TempRoomSetpSetShiftF16_3,
    #[dpt_type(main = 223, sub = 100)]
    DPT_EnergyDemAir,
    #[dpt_type(main = 224, sub = 100)]
    DPT_TempSupplyAirSetpSet,
    #[dpt_type(main = 225, sub = 1)]
    DPT_ScalingSpeed,
    #[dpt_type(main = 225, sub = 2)]
    DPT_Scaling_Step_Time,
    #[dpt_type(main = 225, sub = 3)]
    DPT_TariffNext,
    #[dpt_type(main = 229, sub = 1)]
    DPT_MeteringValue,
    #[dpt_type(main = 231, sub = 1)]
    DPT_Locale_ASCII,
    #[dpt_type(main = 232, sub = 600)]
    DPT_Colour_RGB,
    #[dpt_type(main = 234, sub = 1)]
    DPT_LanguageCodeAlpha2_ASCII,
    #[dpt_type(main = 234, sub = 2)]
    DPT_RegionCodeAlpha2_ASCII,
    #[dpt_type(main = 235, sub = 1)]
    DPT_Tariff_ActiveEnergy,
    #[dpt_type(main = 236, sub = 1)]
    DPT_Prioritised_Mode_Control,
    #[dpt_type(main = 237, sub = 600)]
    DPT_DALI_Control_Gear_Diagnostic,
    #[dpt_type(main = 238, sub = 1)]
    DPT_SceneConfig,
    #[dpt_type(main = 238, sub = 600)]
    DPT_DALI_Diagnostics,
    #[dpt_type(main = 239, sub = 1)]
    DPT_FlaggedScaling,
    #[dpt_type(main = 240, sub = 800)]
    DPT_CombinedPosition,
    #[dpt_type(main = 241, sub = 800)]
    DPT_StatusSAB,
    #[dpt_type(main = 242, sub = 600)]
    DPT_Colour_xyY,
    #[dpt_type(main = 243, sub = 600)]
    DPT_Colour_Transition_xyY,
    #[dpt_type(main = 244, sub = 600)]
    DPT_Converter_Status,
    #[dpt_type(main = 245, sub = 600)]
    DPT_Converter_Test_Result,
    #[dpt_type(main = 246, sub = 600)]
    DPT_Battery_Info,
    #[dpt_type(main = 247, sub = 600)]
    D_PT_Con_verter_T_est_l_nfo,
    #[dpt_type(main = 248, sub = 600)]
    DPT_Converter_Info_Fix,
    #[dpt_type(main = 250, sub = 600)]
    DPT_Brightness_Colour_Temperature_Control,
    #[dpt_type(main = 251, sub = 600)]
    DPT_Colour_RGBW,
    #[dpt_type(main = 252, sub = 600)]
    DPT_Relative_Control_RGBW,
    #[dpt_type(main = 253, sub = 600)]
    DPT_Relative_Control_xyY,
    #[dpt_type(main = 254, sub = 600)]
    DPT_Relative_Control_RGB,
    #[dpt_type(main = 255, sub = 1)]
    DPT_GeographicalLocation,
    #[dpt_type(main = 256, sub = 1)]
    DPT_DateTime_Period,
    #[dpt_type(main = 265, sub = 1)]
    DPT_DateTime_Switch,
    #[dpt_type(main = 265, sub = 5)]
    DPT_DateTime_Alarm,
    #[dpt_type(main = 265, sub = 9)]
    DPT_DateTime_OpenClose,
    #[dpt_type(main = 265, sub = 11)]
    DPT_DateTime_State,
    #[dpt_type(main = 265, sub = 12)]
    DPT_DateTime_Invert,
    #[dpt_type(main = 266, sub = 27)]
    DPT_DateTime_Value_Electric_Potential,
    #[dpt_type(main = 266, sub = 56)]
    DPT_DateTime_Value_Power,
    #[dpt_type(main = 266, sub = 80)]
    DPT_DateTime_Value_ApparentPower,
    #[dpt_type(main = 267, sub = 1)]
    DPT_DateTime_UTF8,
    #[dpt_type(main = 272, sub = 600)]
    DPT_Converter_Info,
    #[dpt_type(main = 273, sub = 1)]
    DPT_Forecast_Temperature,
    #[dpt_type(main = 273, sub = 2)]
    DPT_Forecast_WindSpeed,
    #[dpt_type(main = 273, sub = 3)]
    DPT_Forecast_RelativeHumidity,
    #[dpt_type(main = 273, sub = 4)]
    DPT_Forecast_AbsoluteHumidity,
    #[dpt_type(main = 273, sub = 5)]
    DPT_Forecast_C02,
    #[dpt_type(main = 273, sub = 6)]
    DPT_Forecast_AirPollutants,
    #[dpt_type(main = 273, sub = 7)]
    DPT_Forecast_Sunlntensity,
    #[dpt_type(main = 274, sub = 1)]
    DPT_Forecast_Wind_Direction,
}

#[test]
fn test_dpt_no_std() {
    let a = DPT::DPT_Switch;
    let b = DPT::DPT_Bool;
    let c = DPT::DPT_Value_Temp;

    assert_eq!(1, a.main());
    assert_eq!(1, b.main());
    assert_eq!(9, c.main());

    assert_eq!(1, a.sub());
    assert_eq!(2, b.sub());
    assert_eq!(1, c.sub());
}

#[cfg(feature = "std")]
#[test]
fn test_dpt_std() {
    let a = DPT::DPT_Switch;
    let b = DPT::DPT_Bool;
    let c = DPT::DPT_Value_Temp;

    assert_eq!("DPST-1-1", a.dpst());
    assert_eq!("DPST-1-2", b.dpst());
    assert_eq!("DPST-9-1", c.dpst());

    assert_eq!(DPT::from_dpst("DPST-1-1").unwrap(), DPT::DPT_Switch);
    assert_eq!(DPT::from_dpst("DPST-1-2").unwrap(), DPT::DPT_Bool);
    assert_eq!(DPT::from_dpst("DPST-9-1").unwrap(), DPT::DPT_Value_Temp);
}