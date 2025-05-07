#[derive(Debug, Clone, PartialEq)]
pub enum UFEnum {
    AC,
    AL,
    AP,
    AM,
    BA,
    CE,
    DF,
    ES,
    GO,
    MA,
    MT,
    MS,
    MG,
    PA,
    PB,
    PR,
    PE,
    PI,
    RJ,
    RN,
    RS,
    RO,
    RR,
    SC,
    SP,
    SE,
    TO,
    BRAZIL,
}

#[derive(Debug, Clone)]
pub struct UF {
    uf: UFEnum,
    min_max_latitude: (f32, f32),
    min_max_longitude: (f32, f32),
}

impl UF {
    pub fn get_uf_enum(&self) -> &UFEnum {
        &self.uf
    }

    pub fn get_min_max_latitude(&self) -> &(f32, f32) {
        &self.min_max_latitude
    }

    pub fn get_min_max_longitude(&self) -> &(f32, f32) {
        &self.min_max_longitude
    }

    pub fn get_uf_from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::brazil()),
            12 => Some(Self::ac()),
            27 => Some(Self::al()),
            16 => Some(Self::ap()),
            13 => Some(Self::am()),
            29 => Some(Self::ba()),
            23 => Some(Self::ce()),
            53 => Some(Self::df()),
            32 => Some(Self::es()),
            52 => Some(Self::go()),
            21 => Some(Self::ma()),
            51 => Some(Self::mt()),
            50 => Some(Self::ms()),
            31 => Some(Self::mg()),
            15 => Some(Self::pa()),
            25 => Some(Self::pb()),
            41 => Some(Self::pr()),
            26 => Some(Self::pe()),
            22 => Some(Self::pi()),
            33 => Some(Self::rj()),
            24 => Some(Self::rn()),
            43 => Some(Self::rs()),
            11 => Some(Self::ro()),
            14 => Some(Self::rr()),
            42 => Some(Self::sc()),
            35 => Some(Self::sp()),
            28 => Some(Self::se()),
            17 => Some(Self::to()),
            _ => None,
        }
    }

    pub fn get_uf_from_str(str: &str) -> Option<Self> {
        match str.to_uppercase().as_str() {
            "BRAZIL" => Some(Self::brazil()),
            "AC" => Some(Self::ac()),
            "AL" => Some(Self::al()),
            "AP" => Some(Self::ap()),
            "AM" => Some(Self::am()),
            "BA" => Some(Self::ba()),
            "CE" => Some(Self::ce()),
            "DF" => Some(Self::df()),
            "ES" => Some(Self::es()),
            "GO" => Some(Self::go()),
            "MA" => Some(Self::ma()),
            "MT" => Some(Self::mt()),
            "MS" => Some(Self::ms()),
            "MG" => Some(Self::mg()),
            "PA" => Some(Self::pa()),
            "PB" => Some(Self::pb()),
            "PR" => Some(Self::pr()),
            "PE" => Some(Self::pe()),
            "PI" => Some(Self::pi()),
            "RJ" => Some(Self::rj()),
            "RN" => Some(Self::rn()),
            "RS" => Some(Self::rs()),
            "RO" => Some(Self::ro()),
            "RR" => Some(Self::rr()),
            "SC" => Some(Self::sc()),
            "SP" => Some(Self::sp()),
            "SE" => Some(Self::se()),
            "TO" => Some(Self::to()),
            _ => None,
        }
    }

    fn brazil() -> Self {
        Self {
            uf: UFEnum::BRAZIL,
            min_max_latitude: (-33.6866, 4.60314),
            min_max_longitude: (-72.8997, -35.2267),
        }
    }

    fn ac() -> Self {
        Self {
            uf: UFEnum::AC,
            min_max_latitude: (-11.0188, -7.61657),
            min_max_longitude: (-72.8997, -66.8972),
        }
    }

    fn al() -> Self {
        Self {
            uf: UFEnum::AL,
            min_max_latitude: (-10.406, -8.83951),
            min_max_longitude: (-37.9988, -35.2267),
        }
    }

    fn ap() -> Self {
        Self {
            uf: UFEnum::AP,
            min_max_latitude: (-0.938, 3.84074),
            min_max_longitude: (-52.453, -50.6996),
        }
    }

    fn am() -> Self {
        Self {
            uf: UFEnum::AM,
            min_max_latitude: (-8.74232, -0.11909),
            min_max_longitude: (-72.5907, -56.7112),
        }
    }

    fn ba() -> Self {
        Self {
            uf: UFEnum::BA,
            min_max_latitude: (-18.0754, -8.72073),
            min_max_longitude: (-45.7866, -37.6131),
        }
    }

    fn ce() -> Self {
        Self {
            uf: UFEnum::CE,
            min_max_latitude: (-7.82163, -2.79331),
            min_max_longitude: (-41.2435, -37.3531),
        }
    }

    fn df() -> Self {
        Self {
            uf: UFEnum::DF,
            min_max_latitude: (-15.7795, -15.7795),
            min_max_longitude: (-47.9297, -47.9297),
        }
    }

    fn es() -> Self {
        Self {
            uf: UFEnum::ES,
            min_max_latitude: (-21.1523, -18.0965),
            min_max_longitude: (-41.8405, -39.7362),
        }
    }

    fn go() -> Self {
        Self {
            uf: UFEnum::GO,
            min_max_latitude: (-19.1832, -13.035),
            min_max_longitude: (-53.2012, -46.1165),
        }
    }

    fn ma() -> Self {
        Self {
            uf: UFEnum::MA,
            min_max_latitude: (-9.10273, -1.19696),
            min_max_longitude: (-48.4291, -41.905),
        }
    }

    fn mt() -> Self {
        Self {
            uf: UFEnum::MT,
            min_max_latitude: (-17.8241, -9.46121),
            min_max_longitude: (-61.4697, -50.514),
        }
    }

    fn ms() -> Self {
        Self {
            uf: UFEnum::MS,
            min_max_latitude: (-23.9705, -17.5698),
            min_max_longitude: (-57.8836, -51.0961),
        }
    }

    fn mg() -> Self {
        Self {
            uf: UFEnum::MG,
            min_max_latitude: (-22.854, -14.2662),
            min_max_longitude: (-50.6894, -39.9391),
        }
    }

    fn pa() -> Self {
        Self {
            uf: UFEnum::PA,
            min_max_latitude: (-9.3281, -0.154874),
            min_max_longitude: (-57.7544, -46.1399),
        }
    }

    fn pb() -> Self {
        Self {
            uf: UFEnum::PB,
            min_max_latitude: (-8.15289, -6.18515),
            min_max_longitude: (-38.676, -34.8151),
        }
    }

    fn pr() -> Self {
        Self {
            uf: UFEnum::PR,
            min_max_latitude: (-26.4839, -22.5523),
            min_max_longitude: (-54.5827, -48.3204),
        }
    }

    fn pe() -> Self {
        Self {
            uf: UFEnum::PE,
            min_max_latitude: (-9.38866, -3.8396),
            min_max_longitude: (-41.0095, -32.4107),
        }
    }

    fn pi() -> Self {
        Self {
            uf: UFEnum::PI,
            min_max_latitude: (-10.817, -2.85774),
            min_max_longitude: (-45.9116, -40.6083),
        }
    }

    fn rj() -> Self {
        Self {
            uf: UFEnum::RJ,
            min_max_latitude: (-23.2221, -20.9276),
            min_max_longitude: (-44.7175, -41.0446),
        }
    }

    fn rn() -> Self {
        Self {
            uf: UFEnum::RN,
            min_max_latitude: (-6.93835, -4.83729),
            min_max_longitude: (-38.4947, -35.0033),
        }
    }

    fn rs() -> Self {
        Self {
            uf: UFEnum::RS,
            min_max_latitude: (-33.6866, -27.1607),
            min_max_longitude: (-57.5497, -49.7333),
        }
    }

    fn ro() -> Self {
        Self {
            uf: UFEnum::RO,
            min_max_latitude: (-13.4945, -8.76077),
            min_max_longitude: (-65.3346, -60.1488),
        }
    }

    fn rr() -> Self {
        Self {
            uf: UFEnum::RR,
            min_max_latitude: (0.884203, 4.60314),
            min_max_longitude: (-61.3692, -59.6204),
        }
    }

    fn sc() -> Self {
        Self {
            uf: UFEnum::SC,
            min_max_latitude: (-29.3099, -26.0292),
            min_max_longitude: (-53.7166, -48.5146),
        }
    }

    fn sp() -> Self {
        Self {
            uf: UFEnum::SP,
            min_max_latitude: (-25.0144, -19.9453),
            min_max_longitude: (-53.0603, -44.3281),
        }
    }

    fn se() -> Self {
        Self {
            uf: UFEnum::SE,
            min_max_latitude: (-11.5157, -9.64882),
            min_max_longitude: (-38.1813, -36.4611),
        }
    }

    fn to() -> Self {
        Self {
            uf: UFEnum::TO,
            min_max_latitude: (-13.0447, -5.26131),
            min_max_longitude: (-49.9521, -46.4076),
        }
    }
}
