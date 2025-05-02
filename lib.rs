#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address, Vec, String, Map, map, log};

// Certificate metadata
#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub cert_id: Symbol,
    pub recipient: Address,
    pub issuer: Address,
    pub course: String,
    pub issued_at: u64,
}

#[contracttype]
pub enum CertKey {
    CertById(Symbol),
    CertsOf(Address),
}

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {
    // Issue a new certificate to a recipient
    pub fn issue_cert(env: Env, issuer: Address, recipient: Address, cert_id: Symbol, course: String) {
        issuer.require_auth();

        let cert = Certificate {
            cert_id: cert_id.clone(),
            recipient: recipient.clone(),
            issuer: issuer.clone(),
            course,
            issued_at: env.ledger().timestamp(),
        };

        env.storage().instance().set(&CertKey::CertById(cert_id.clone()), &cert);

        // Append cert to recipient's list
        let mut certs: Vec<Symbol> = env.storage().instance().get(&CertKey::CertsOf(recipient.clone())).unwrap_or(Vec::new(&env));
        certs.push_back(cert_id.clone());
        env.storage().instance().set(&CertKey::CertsOf(recipient), &certs);

        log!(&env, "Certificate {} issued to {:?}", cert_id, cert.recipient);
    }

    // Retrieve certificate by ID
    pub fn get_cert(env: Env, cert_id: Symbol) -> Option<Certificate> {
        env.storage().instance().get(&CertKey::CertById(cert_id))
    }

    // Retrieve all certificate IDs for an address
    pub fn list_certs(env: Env, recipient: Address) -> Vec<Symbol> {
        env.storage().instance().get(&CertKey::CertsOf(recipient)).unwrap_or(Vec::new(&env))
    }
}
