use crate::mode::Mode;
use quote::quote;

pub(crate) fn mode_params_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mode = syn::parse_macro_input!(input as Mode);
    let mode_name = mode.mode_name;
    let mode_name_str = format!("{}", mode_name).to_lowercase();

    pub const Q: usize = 8380417;

    let (
        k,
        l,
        eta,
        tau,
        beta,
        gamma1,
        gamma2,
        omega,
        polyz_packedbytes,
        polyw1_packedbytes,
        polyeta_packedbytes,
    ) = match mode_name_str.as_str() {
        "mode2" => (
            4usize,
            4usize,
            2usize,
            39usize,
            78usize,
            (1 << 17) as usize,
            (Q - 1) / 88,
            80usize,
            576usize,
            192usize,
            96usize,
        ),
        "mode3" => (
            6usize,
            5usize,
            4usize,
            49usize,
            196usize,
            (1 << 19) as usize,
            (Q - 1) / 32,
            55usize,
            640usize,
            128usize,
            128usize,
        ),
        "mode5" => (
            8usize,
            7usize,
            2usize,
            60usize,
            120usize,
            (1 << 19) as usize,
            (Q - 1) / 32,
            75usize,
            640usize,
            128usize,
            96usize,
        ),
        _ => panic!("Mode must be mode2, mode3 or mode5"),
    };

    let tokens = quote! {
      pub mod params {
        use crate::#mode_name::symmetric::STREAM256_BLOCKBYTES;
        pub const SEEDBYTES: usize = 32;
        pub const CRHBYTES: usize = 64;
        pub const N: usize = 256;
        pub const Q: usize = 8380417;
        pub const D: usize = 13;
        pub const ROOT_OF_UNITY: usize = 1753;

        // begin from params
        pub const K: usize = #k;
        pub const L: usize = #l;
        pub const ETA: usize = #eta;
        pub const TAU: usize = #tau;
        pub const BETA: usize = #beta;
        pub const GAMMA1: usize = #gamma1;
        pub const GAMMA2: usize = #gamma2;
        pub const OMEGA: usize = #omega;
        pub const POLYZ_PACKEDBYTES: usize = #polyz_packedbytes;
        pub const POLYW1_PACKEDBYTES: usize = #polyw1_packedbytes;
        pub const POLYETA_PACKEDBYTES: usize = #polyeta_packedbytes;

        const POLY_UNIFORM_GAMMA1_NBLOCKS: usize =
            (POLYZ_PACKEDBYTES + STREAM256_BLOCKBYTES - 1) / STREAM256_BLOCKBYTES;

        pub const SECRETKEYBYTES: usize = 3 * SEEDBYTES
            + L * POLYETA_PACKEDBYTES
            + K * POLYETA_PACKEDBYTES
            + K * POLYT0_PACKEDBYTES;
        pub const SIGNBYTES: usize =
            SEEDBYTES + L * POLYZ_PACKEDBYTES + POLYVECH_PACKEDBYTES;
        // end from params

        pub const POLYT1_PACKEDBYTES: usize = 320;
        pub const POLYT0_PACKEDBYTES: usize = 416;
        pub const POLYVECH_PACKEDBYTES: usize = OMEGA + K;

        // Concise types to avoid cast cluttering
        pub const Q_I32: i32 = Q as i32;
        pub const N_U32: u32 = N as u32;
        pub const L_U16: u16 = L as u16;
        pub const BETA_I32: i32 = BETA as i32;
        pub const GAMMA1_I32: i32 = GAMMA1 as i32;
        pub const GAMMA2_I32: i32 = GAMMA2 as i32;
        pub const OMEGA_U8: u8 = OMEGA as u8;
        pub const ETA_I32: i32 = ETA as i32;
        pub const GAMMA1_SUB_BETA: i32 = (GAMMA1 - BETA) as i32;

        pub const PUBLICKEYBYTES: usize = SEEDBYTES + K * POLYT1_PACKEDBYTES;
        // pub const SECRETKEYBYTES: usize = 3 * SEEDBYTES
        //     + L * POLYETA_PACKEDBYTES
        //     + K * POLYETA_PACKEDBYTES
        //     + K * POLYT0_PACKEDBYTES;
        // pub const SIGNBYTES: usize =
        //   SEEDBYTES + L * POLYZ_PACKEDBYTES + POLYVECH_PACKEDBYTES;

        pub const RANDOMIZED_SIGNING: bool = cfg!(feature = "random_signing");
      }
    };
    tokens.into()
}
