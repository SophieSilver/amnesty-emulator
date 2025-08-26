mod prepare;

pub mod read;

macro_rules! test_addressing_modes {
    (
        instruction: $instruction:ident,
        instruction_type: $instruction_type:ident,
        addressing_modes: [
            $($addressing_mode:ident),+
            $(,)?
        ]$(,)?
    ) => {
        $(
        paste::paste! {
            const _: () = {
                use $crate::cpu::tests::addressing_modes::[<$instruction_type:lower>]::[<Test $instruction_type:camel $addressing_mode:camel>];
                use $crate::cpu::instructions::opcode::OpCode;

                impl [<Test $instruction_type:camel $addressing_mode:camel>] for $instruction {
                    const OPCODE: OpCode = OpCode::[<$instruction:camel $addressing_mode:camel>];
                }
            };

            test_addressing_modes! {
                @inner write_test {
                    $instruction,
                    $instruction_type,
                    [<$addressing_mode:camel>],
                }
            }
        }
        )+
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            ZeropageX,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                ZeropageX,
                [ zeropage_x, zeropage_x_overflow ]
            }
        }
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            ZeropageY,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                ZeropageY,
                [ zeropage_y, zeropage_y_overflow ]
            }
        }
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            AbsoluteX,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                AbsoluteX,
                [ absolute_x, absolute_x_overflow ]
            }
        }
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            AbsoluteY,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                AbsoluteY,
                [ absolute_y, absolute_y_overflow ]
            }
        }
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            IndirectX,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                IndirectX,
                [ indirect_x, indirect_x_overflow, indirect_x_page_split ]
            }
        }
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            IndirectY,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                IndirectY,
                [ indirect_y, indirect_y_overflow, indirect_y_page_split ]
            }
        }
    };

    (
        @inner write_test {
            $instruction:ident,
            $instruction_type:ident,
            $addressing_mode:ident,
        }
    ) => {
        test_addressing_modes! {
            @inner test_fns {
                $instruction,
                $instruction_type,
                $addressing_mode,
                [ $addressing_mode ]
            }
        }
    };

    (
        @inner test_fns {
            $instruction:ident,
            $instruction_type:ident,
            $trait:ident,
            [ $($fn_name:ident),+ ]
        }
    ) => {
        paste::paste! {
            $(
                #[test]
                fn [<$fn_name:snake>]() {
                    use $crate::cpu::tests::addressing_modes::[<$instruction_type:lower>]::[<Test $instruction_type:camel $trait:camel>];
                    <$instruction as [<Test $instruction_type:camel $trait:camel>]>::[<test_ $fn_name:snake>]();
                }
            )+
        }
    }
}

pub(crate) use test_addressing_modes;
