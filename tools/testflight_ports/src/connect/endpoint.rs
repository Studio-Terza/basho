use std::fmt;

pub enum Endpoint<'name> {
    DistributionCertificates,
    Certificates,
    BashoBundle,
    Profiles,
    ProfilesNamed(&'name str),
    Profile(&'name str),
}

impl Endpoint<'_> {
    const API: &str = "https://api.appstoreconnect.apple.com/v1/";
}

impl fmt::Display for Endpoint<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(Self::API)?;
        match self {
            Endpoint::DistributionCertificates => {
                formatter.write_str("certificates?filter[certificateType]=DISTRIBUTION&limit=200")
            }
            Endpoint::Certificates => formatter.write_str("certificates"),
            Endpoint::BashoBundle => {
                formatter.write_str("bundleIds?filter[identifier]=com.cinglein.basho")
            }
            Endpoint::Profiles => formatter.write_str("profiles"),
            Endpoint::ProfilesNamed(name) => {
                write!(
                    formatter,
                    "profiles?filter[name]={}",
                    name.replace(' ', "%20")
                )
            }
            Endpoint::Profile(identifier) => write!(formatter, "profiles/{identifier}"),
        }
    }
}
