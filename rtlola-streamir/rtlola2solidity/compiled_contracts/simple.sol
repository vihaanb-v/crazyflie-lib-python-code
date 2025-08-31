contract Contract {
    uint64[1] a_buffer;
    bool[1] a_valid;
    uint64 a_current;
    uint64[1] b_buffer;
    bool[1] b_valid;
    uint64 b_current;
    string[1] trigger_0_buffer;
    bool[1] trigger_0_valid;
    uint64 trigger_0_current;
    function eval_a(uint64 value) private {
        a_buffer[a_current] = value;
        a_valid[a_current] = true;
    }

    function eval_b_0(uint64 a) private returns (uint64 b) {
        b = (a + uint64(3));
        b_buffer[b_current] = b;
        b_valid[b_current] = true;
    }

    function shift_a() private {
        a_current = (a_current + 1) % 0;
    }

    function shift_b() private {
        b_current = (b_current + 1) % 0;
    }

    function shift_trigger_0() private {
        trigger_0_current = (trigger_0_current + 1) % 0;
    }

    event Trigger0();
    function trigger0() private {
        //
        emit Trigger0();
    }

    function simple(uint64 a) public {
        shift_a();
        eval_a(a);
        shift_b();
        uint64 b = eval_b_0(a);
        if ((b > uint64(3))) {
            shift_trigger_0();
        }
        if ((b > uint64(3))) {
            trigger0();
        }
    }
}
