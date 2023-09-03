#[cfg(test)]
mod benchmark {
    use clap_verbosity_flag::Verbosity;
    use heimdall_common::testing::benchmarks::benchmark;

    use heimdall::decode::DecodeArgs;

    #[test]
    fn benchmark_decode_transfer() {
        fn bench() {
            let args = DecodeArgs {
                target: String::from("0xc47f00270000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000b6a6265636b65722e657468000000000000000000000000000000000000000000"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
            };
            heimdall::decode::decode(args)
        }

        benchmark("benchmark_decode_transfer", 100, bench)
    }

    #[test]
    fn benchmark_decode_uniswap_simple() {
        fn bench() {
            let args = DecodeArgs {
                target: String::from("791ac947000000000000000000000000000000000000000000ac03e3c2829679f93600000000000000000000000000000000000000000000000000000200b952bc426b0c00000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000bffadf2903e6ec32dc0a7b5a2b57de0e728ec0b500000000000000000000000000000000000000000000000000000000645c1321000000000000000000000000000000000000000000000000000000000000000200000000000000000000000039702bb7c6a482bdde6aec96175bc6cdc277b999000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
            };
            heimdall::decode::decode(args)
        }

        benchmark("benchmark_decode_uniswap_simple", 100, bench)
    }

    #[test]
    fn benchmark_decode_seaport_simple() {
        fn bench() {
            let args = DecodeArgs {
                target: String::from("0xfb0f3ee100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec9c58de0a8000000000000000000000000000d2f8a98bde7c701ae961d10d0d1fc3a751be737f000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c000000000000000000000000005008c2a3af41024e9f0bd0432df4f75828602598000000000000000000000000000000000000000000000000000000000000110600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000006358934b00000000000000000000000000000000000000000000000000000000637e22710000000000000000000000000000000000000000000000000000000000000000360c6ebe000000000000000000000000000000000000000038844ef19f04aecf0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000066517289880000000000000000000000000000000a26b00c1f0df003000390027140000faa719000000000000000000000000000000000000000000000000000cca2e51310000000000000000000000000000cecf12f47d2896c90f6e19b7376fa3b169fabd920000000000000000000000000000000000000000000000000000000000000041447858c6d8251fb8ffba546bedb410457ff77148fdf59ac8e046993936a134b028f535c5b1f760508b6e0c3c18d44927d82da0502c66688c0dc961a434a9b0071c00000000000000000000000000000000000000000000000000000000000000"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
            };
            heimdall::decode::decode(args)
        }

        benchmark("benchmark_decode_seaport_simple", 100, bench)
    }

    #[test]
    fn benchmark_decode_seaport_complex() {
        fn bench() {
            let args = DecodeArgs {
                target: String::from("0xc47f00270000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000b6a6265636b65722e657468000000000000000000000000000000000000000000"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
            };
            heimdall::decode::decode(args)
        }

        benchmark("benchmark_decode_seaport_complex", 100, bench)
    }

    #[test]
    fn test_decode_transfer() {
        let args = DecodeArgs {
            target: String::from("0xfb0f3ee100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec9c58de0a8000000000000000000000000000d2f8a98bde7c701ae961d10d0d1fc3a751be737f000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c000000000000000000000000005008c2a3af41024e9f0bd0432df4f75828602598000000000000000000000000000000000000000000000000000000000000110600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000006358934b00000000000000000000000000000000000000000000000000000000637e22710000000000000000000000000000000000000000000000000000000000000000360c6ebe000000000000000000000000000000000000000038844ef19f04aecf0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000066517289880000000000000000000000000000000a26b00c1f0df003000390027140000faa719000000000000000000000000000000000000000000000000000cca2e51310000000000000000000000000000cecf12f47d2896c90f6e19b7376fa3b169fabd920000000000000000000000000000000000000000000000000000000000000041447858c6d8251fb8ffba546bedb410457ff77148fdf59ac8e046993936a134b028f535c5b1f760508b6e0c3c18d44927d82da0502c66688c0dc961a434a9b0071c00000000000000000000000000000000000000000000000000000000000000"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from(""),
            openai_api_key: String::from(""),
            explain: false,
            default: true,
            truncate_calldata: false,
        };
        heimdall::decode::decode(args);
        assert!(true)
    }

    #[test]
    fn test_decode_seaport_simple() {
        let args = DecodeArgs {
            target: String::from("0xfb0f3ee100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec9c58de0a8000000000000000000000000000d2f8a98bde7c701ae961d10d0d1fc3a751be737f000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c000000000000000000000000005008c2a3af41024e9f0bd0432df4f75828602598000000000000000000000000000000000000000000000000000000000000110600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000006358934b00000000000000000000000000000000000000000000000000000000637e22710000000000000000000000000000000000000000000000000000000000000000360c6ebe000000000000000000000000000000000000000038844ef19f04aecf0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000066517289880000000000000000000000000000000a26b00c1f0df003000390027140000faa719000000000000000000000000000000000000000000000000000cca2e51310000000000000000000000000000cecf12f47d2896c90f6e19b7376fa3b169fabd920000000000000000000000000000000000000000000000000000000000000041447858c6d8251fb8ffba546bedb410457ff77148fdf59ac8e046993936a134b028f535c5b1f760508b6e0c3c18d44927d82da0502c66688c0dc961a434a9b0071c00000000000000000000000000000000000000000000000000000000000000"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from(""),
            openai_api_key: String::from(""),
            explain: false,
            default: true,
            truncate_calldata: false,
        };
        heimdall::decode::decode(args);
        assert!(true)
    }
}
