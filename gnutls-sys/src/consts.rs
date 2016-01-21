#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
use libc::{c_int};

pub const GNUTLS_E_SUCCESS: c_int = 0;
pub const GNUTLS_E_UNKNOWN_COMPRESSION_ALGORITHM: c_int = -3;
pub const GNUTLS_E_UNKNOWN_CIPHER_TYPE: c_int = -6;
pub const GNUTLS_E_LARGE_PACKET: c_int = -7;

/* GNUTLS_A_PROTOCOL_VERSION */
pub const GNUTLS_E_UNSUPPORTED_VERSION_PACKET: c_int = -8;

/* GNUTLS_A_RECORD_OVERFLOW */
pub const GNUTLS_E_UNEXPECTED_PACKET_LENGTH: c_int = -9;
pub const GNUTLS_E_INVALID_SESSION: c_int = -10;
pub const GNUTLS_E_FATAL_ALERT_RECEIVED: c_int = -12;

/* GNUTLS_A_UNEXPECTED_MESSAGE */
pub const GNUTLS_E_UNEXPECTED_PACKET: c_int = -15;
pub const GNUTLS_E_WARNING_ALERT_RECEIVED: c_int = -16;
pub const GNUTLS_E_ERROR_IN_FINISHED_PACKET: c_int = -18;
pub const GNUTLS_E_UNEXPECTED_HANDSHAKE_PACKET: c_int = -19;

/* GNUTLS_A_HANDSHAKE_FAILURE */
pub const GNUTLS_E_UNKNOWN_CIPHER_SUITE: c_int = -21;
pub const GNUTLS_E_UNWANTED_ALGORITHM: c_int = -22;
pub const GNUTLS_E_MPI_SCAN_FAILED: c_int = -23;

/* GNUTLS_A_DECRYPTION_FAILED, GNUTLS_A_BAD_RECORD_MAC */
pub const GNUTLS_E_DECRYPTION_FAILED: c_int = -24;
pub const GNUTLS_E_MEMORY_ERROR: c_int = -25;

/* GNUTLS_A_DECOMPRESSION_FAILURE */
pub const GNUTLS_E_DECOMPRESSION_FAILED: c_int = -26;
pub const GNUTLS_E_COMPRESSION_FAILED: c_int = -27;
pub const GNUTLS_E_AGAIN: c_int = -28;
pub const GNUTLS_E_EXPIRED: c_int = -29;
pub const GNUTLS_E_DB_ERROR: c_int = -30;
pub const GNUTLS_E_SRP_PWD_ERROR: c_int = -31;
pub const GNUTLS_E_INSUFFICIENT_CREDENTIALS: c_int = -32;

/* For backwards compatibility only. */
pub const GNUTLS_E_INSUFICIENT_CREDENTIALS: c_int = GNUTLS_E_INSUFFICIENT_CREDENTIALS;
pub const GNUTLS_E_INSUFFICIENT_CRED: c_int = GNUTLS_E_INSUFFICIENT_CREDENTIALS;
pub const GNUTLS_E_INSUFICIENT_CRED: c_int = GNUTLS_E_INSUFFICIENT_CREDENTIALS;

pub const GNUTLS_E_HASH_FAILED: c_int = -33;
pub const GNUTLS_E_BASE64_DECODING_ERROR: c_int = -34;

pub const GNUTLS_E_MPI_PRINT_FAILED: c_int = -35;

/* GNUTLS_A_NO_RENEGOTIATION */
pub const GNUTLS_E_REHANDSHAKE: c_int = -37;
pub const GNUTLS_E_GOT_APPLICATION_DATA: c_int = -38;
pub const GNUTLS_E_RECORD_LIMIT_REACHED: c_int = -39;
pub const GNUTLS_E_ENCRYPTION_FAILED: c_int = -40;

pub const GNUTLS_E_PK_ENCRYPTION_FAILED: c_int = -44;
pub const GNUTLS_E_PK_DECRYPTION_FAILED: c_int = -45;
pub const GNUTLS_E_PK_SIGN_FAILED: c_int = -46;
pub const GNUTLS_E_X509_UNSUPPORTED_CRITICAL_EXTENSION: c_int = -47;
pub const GNUTLS_E_KEY_USAGE_VIOLATION: c_int = -48;

/* GNUTLS_A_BAD_CERTIFICATE */
pub const GNUTLS_E_NO_CERTIFICATE_FOUND: c_int = -49;
pub const GNUTLS_E_INVALID_REQUEST: c_int = -50;
pub const GNUTLS_E_SHORT_MEMORY_BUFFER: c_int = -51;
pub const GNUTLS_E_INTERRUPTED: c_int = -52;
pub const GNUTLS_E_PUSH_ERROR: c_int = -53;
pub const GNUTLS_E_PULL_ERROR: c_int = -54;

/* GNUTLS_A_ILLEGAL_PARAMETER */
pub const GNUTLS_E_RECEIVED_ILLEGAL_PARAMETER: c_int = -55;
pub const GNUTLS_E_REQUESTED_DATA_NOT_AVAILABLE: c_int = -56;
pub const GNUTLS_E_PKCS1_WRONG_PAD: c_int = -57;
pub const GNUTLS_E_RECEIVED_ILLEGAL_EXTENSION: c_int = -58;
pub const GNUTLS_E_INTERNAL_ERROR: c_int = -59;
pub const GNUTLS_E_DH_PRIME_UNACCEPTABLE: c_int = -63;
pub const GNUTLS_E_FILE_ERROR: c_int = -64;
pub const GNUTLS_E_TOO_MANY_EMPTY_PACKETS: c_int = -78;
pub const GNUTLS_E_UNKNOWN_PK_ALGORITHM: c_int = -80;
pub const GNUTLS_E_TOO_MANY_HANDSHAKE_PACKETS: c_int = -81;

// Returned if you need to generate temporary RSA
// parameters. These are needed for export cipher suites.
pub const GNUTLS_E_NO_TEMPORARY_RSA_PARAMS: c_int = -84;
pub const GNUTLS_E_NO_COMPRESSION_ALGORITHMS: c_int = -86;
pub const GNUTLS_E_NO_CIPHER_SUITES: c_int = -87;
pub const GNUTLS_E_OPENPGP_GETKEY_FAILED: c_int = -88;
pub const GNUTLS_E_PK_SIG_VERIFY_FAILED: c_int = -89;
pub const GNUTLS_E_ILLEGAL_SRP_USERNAME: c_int = -90;
pub const GNUTLS_E_SRP_PWD_PARSING_ERROR: c_int = -91;
pub const GNUTLS_E_NO_TEMPORARY_DH_PARAMS: c_int = -93;

// For certificate and key stuff
pub const GNUTLS_E_ASN1_ELEMENT_NOT_FOUND: c_int = -67;
pub const GNUTLS_E_ASN1_IDENTIFIER_NOT_FOUND: c_int = -68;
pub const GNUTLS_E_ASN1_DER_ERROR: c_int = -69;
pub const GNUTLS_E_ASN1_VALUE_NOT_FOUND: c_int = -70;
pub const GNUTLS_E_ASN1_GENERIC_ERROR: c_int = -71;
pub const GNUTLS_E_ASN1_VALUE_NOT_VALID: c_int = -72;
pub const GNUTLS_E_ASN1_TAG_ERROR: c_int = -73;
pub const GNUTLS_E_ASN1_TAG_IMPLICIT: c_int = -74;
pub const GNUTLS_E_ASN1_TYPE_ANY_ERROR: c_int = -75;
pub const GNUTLS_E_ASN1_SYNTAX_ERROR: c_int = -76;
pub const GNUTLS_E_ASN1_DER_OVERFLOW: c_int = -77;
pub const GNUTLS_E_OPENPGP_UID_REVOKED: c_int = -79;
pub const GNUTLS_E_CERTIFICATE_ERROR: c_int = -43;

pub const GNUTLS_E_X509_CERTIFICATE_ERROR: c_int = GNUTLS_E_CERTIFICATE_ERROR;
pub const GNUTLS_E_CERTIFICATE_KEY_MISMATCH: c_int = -60;

/* GNUTLS_A_UNSUPPORTED_CERTIFICATE */
pub const GNUTLS_E_UNSUPPORTED_CERTIFICATE_TYPE: c_int = -61;
pub const GNUTLS_E_X509_UNKNOWN_SAN: c_int = -62;
pub const GNUTLS_E_OPENPGP_FINGERPRINT_UNSUPPORTED: c_int = -94;
pub const GNUTLS_E_X509_UNSUPPORTED_ATTRIBUTE: c_int = -95;
pub const GNUTLS_E_UNKNOWN_HASH_ALGORITHM: c_int = -96;
pub const GNUTLS_E_UNKNOWN_PKCS_CONTENT_TYPE: c_int = -97;
pub const GNUTLS_E_UNKNOWN_PKCS_BAG_TYPE: c_int = -98;
pub const GNUTLS_E_INVALID_PASSWORD: c_int = -99;

/* for PKCS #12 MAC */
pub const GNUTLS_E_MAC_VERIFY_FAILED: c_int = -100;
pub const GNUTLS_E_CONSTRAINT_ERROR: c_int = -101;

pub const GNUTLS_E_WARNING_IA_IPHF_RECEIVED: c_int = -102;
pub const GNUTLS_E_WARNING_IA_FPHF_RECEIVED: c_int = -103;

pub const GNUTLS_E_IA_VERIFY_FAILED: c_int = -104;
pub const GNUTLS_E_UNKNOWN_ALGORITHM: c_int = -105;
pub const GNUTLS_E_UNSUPPORTED_SIGNATURE_ALGORITHM: c_int = -106;
pub const GNUTLS_E_SAFE_RENEGOTIATION_FAILED: c_int = -107;
pub const GNUTLS_E_UNSAFE_RENEGOTIATION_DENIED: c_int = -108;
pub const GNUTLS_E_UNKNOWN_SRP_USERNAME: c_int = -109;
pub const GNUTLS_E_PREMATURE_TERMINATION: c_int = -110;

pub const GNUTLS_E_BASE64_ENCODING_ERROR: c_int = -201;

/* obsolete */
pub const GNUTLS_E_INCOMPATIBLE_GCRYPT_LIBRARY: c_int = -202;
pub const GNUTLS_E_INCOMPATIBLE_CRYPTO_LIBRARY: c_int = -202;
pub const GNUTLS_E_INCOMPATIBLE_LIBTASN1_LIBRARY: c_int = -203;

pub const GNUTLS_E_OPENPGP_KEYRING_ERROR: c_int = -204;
pub const GNUTLS_E_X509_UNSUPPORTED_OID: c_int = -205;

pub const GNUTLS_E_RANDOM_FAILED: c_int = -206;
pub const GNUTLS_E_BASE64_UNEXPECTED_HEADER_ERROR: c_int = -207;

pub const GNUTLS_E_OPENPGP_SUBKEY_ERROR: c_int = -208;

pub const GNUTLS_E_CRYPTO_ALREADY_REGISTERED: c_int = GNUTLS_E_ALREADY_REGISTERED;
pub const GNUTLS_E_ALREADY_REGISTERED: c_int = -209;

pub const GNUTLS_E_HANDSHAKE_TOO_LARGE: c_int = -210;

pub const GNUTLS_E_CRYPTODEV_IOCTL_ERROR: c_int = -211;
pub const GNUTLS_E_CRYPTODEV_DEVICE_ERROR: c_int = -212;

pub const GNUTLS_E_CHANNEL_BINDING_NOT_AVAILABLE: c_int = -213;
pub const GNUTLS_E_BAD_COOKIE: c_int = -214;
pub const GNUTLS_E_OPENPGP_PREFERRED_KEY_ERROR: c_int = -215;
pub const GNUTLS_E_INCOMPAT_DSA_KEY_WITH_TLS_PROTOCOL: c_int = -216;
pub const GNUTLS_E_INSUFFICIENT_SECURITY: c_int = -217;

pub const GNUTLS_E_HEARTBEAT_PONG_RECEIVED: c_int = -292;
pub const GNUTLS_E_HEARTBEAT_PING_RECEIVED: c_int = -293;

pub const GNUTLS_E_PKCS11_ERROR: c_int = -300;
pub const GNUTLS_E_PKCS11_LOAD_ERROR: c_int = -301;
pub const GNUTLS_E_PARSING_ERROR: c_int = -302;
pub const GNUTLS_E_PKCS11_PIN_ERROR: c_int = -303;

pub const GNUTLS_E_PKCS11_SLOT_ERROR: c_int = -305;
pub const GNUTLS_E_LOCKING_ERROR: c_int = -306;
pub const GNUTLS_E_PKCS11_ATTRIBUTE_ERROR: c_int = -307;
pub const GNUTLS_E_PKCS11_DEVICE_ERROR: c_int = -308;
pub const GNUTLS_E_PKCS11_DATA_ERROR: c_int = -309;
pub const GNUTLS_E_PKCS11_UNSUPPORTED_FEATURE_ERROR: c_int = -310;
pub const GNUTLS_E_PKCS11_KEY_ERROR: c_int = -311;
pub const GNUTLS_E_PKCS11_PIN_EXPIRED: c_int = -312;
pub const GNUTLS_E_PKCS11_PIN_LOCKED: c_int = -313;
pub const GNUTLS_E_PKCS11_SESSION_ERROR: c_int = -314;
pub const GNUTLS_E_PKCS11_SIGNATURE_ERROR: c_int = -315;
pub const GNUTLS_E_PKCS11_TOKEN_ERROR: c_int = -316;
pub const GNUTLS_E_PKCS11_USER_ERROR: c_int = -317;

pub const GNUTLS_E_CRYPTO_INIT_FAILED: c_int = -318;
pub const GNUTLS_E_TIMEDOUT: c_int = -319;
pub const GNUTLS_E_USER_ERROR: c_int = -320;
pub const GNUTLS_E_ECC_NO_SUPPORTED_CURVES: c_int = -321;
pub const GNUTLS_E_ECC_UNSUPPORTED_CURVE: c_int = -322;
pub const GNUTLS_E_PKCS11_REQUESTED_OBJECT_NOT_AVAILBLE: c_int = -323;
pub const GNUTLS_E_CERTIFICATE_LIST_UNSORTED: c_int = -324;
pub const GNUTLS_E_ILLEGAL_PARAMETER: c_int = -325;
pub const GNUTLS_E_NO_PRIORITIES_WERE_SET: c_int = -326;
pub const GNUTLS_E_X509_UNSUPPORTED_EXTENSION: c_int = -327;
pub const GNUTLS_E_SESSION_EOF: c_int = -328;

pub const GNUTLS_E_TPM_ERROR: c_int = -329;
pub const GNUTLS_E_TPM_KEY_PASSWORD_ERROR: c_int = -330;
pub const GNUTLS_E_TPM_SRK_PASSWORD_ERROR: c_int = -331;
pub const GNUTLS_E_TPM_SESSION_ERROR: c_int = -332;
pub const GNUTLS_E_TPM_KEY_NOT_FOUND: c_int = -333;
pub const GNUTLS_E_TPM_UNINITIALIZED: c_int = -334;
pub const GNUTLS_E_TPM_NO_LIB: c_int = -335;

pub const GNUTLS_E_NO_CERTIFICATE_STATUS: c_int = -340;
pub const GNUTLS_E_OCSP_RESPONSE_ERROR: c_int = -341;
pub const GNUTLS_E_RANDOM_DEVICE_ERROR: c_int = -342;
pub const GNUTLS_E_AUTH_ERROR: c_int = -343;
pub const GNUTLS_E_NO_APPLICATION_PROTOCOL: c_int = -344;
pub const GNUTLS_E_SOCKETS_INIT_ERROR: c_int = -345;
pub const GNUTLS_E_KEY_IMPORT_FAILED: c_int = -346;

/* GNUTLS_A_INAPPROPRIATE_FALLBACK */
pub const GNUTLS_E_INAPPROPRIATE_FALLBACK: c_int = -347;
pub const GNUTLS_E_CERTIFICATE_VERIFICATION_ERROR: c_int = -348;

pub const GNUTLS_E_SELF_TEST_ERROR: c_int = -400;
pub const GNUTLS_E_NO_SELF_TEST: c_int = -401;
pub const GNUTLS_E_LIB_IN_ERROR_STATE: c_int = -402;
pub const GNUTLS_E_PK_GENERATION_ERROR: c_int = -403;
pub const GNUTLS_E_IDNA_ERROR: c_int = -404;

pub const GNUTLS_E_NEED_FALLBACK: c_int = -405;

pub const GNUTLS_E_UNIMPLEMENTED_FEATURE: c_int = -1250;

pub const GNUTLS_E_APPLICATION_ERROR_MAX: c_int = -65000;
pub const GNUTLS_E_APPLICATION_ERROR_MIN: c_int = -65500;

pub const GNUTLS_HEARTBEAT_WAIT: c_int = 1;
pub const GNUTLS_RECORD_WAIT: c_int = 1;
