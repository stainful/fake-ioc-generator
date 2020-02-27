use fake::Fake;

pub enum IOCType {
    IP,
    URL,
    Domain,
    Md5,
    Sha1,
}

fn generate_ip() -> String {
    fake::faker::internet::raw::IPv4(fake::locales::EN).fake()
}


fn generate_md5() -> String {
    let text: String = fake::faker::lorem::raw::Sentence(fake::locales::EN, 2..10).fake();
    let digest = md5::compute(text.as_bytes());

    format!("{:?}", digest)
}

fn generate_sha1() -> String {
    let text: String = fake::faker::lorem::raw::Sentence(fake::locales::EN, 2..10).fake();
    let digest = sha1::Sha1::from(text.as_bytes()).digest();

    digest.to_string().to_ascii_uppercase()
}

fn generate_domain() -> String {
  let mut company: String = fake::faker::company::raw::CompanyName(fake::locales::EN).fake();
  company = company.to_ascii_lowercase();
  company.retain(|c| !c.is_whitespace());
  let domain: String = fake::faker::internet::raw::DomainSuffix(fake::locales::EN).fake();
  format!("{}.{}", company, domain)
}

pub fn generate_fake_ioc(ioc_type: IOCType) -> String {
    match ioc_type {
        IOCType::IP => generate_ip(),
        IOCType::Domain => generate_domain(),
        IOCType::Md5 => generate_md5(),
        IOCType::Sha1 => generate_sha1(),
        _ => unimplemented!(),
    }
}
