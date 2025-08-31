contract Contract {
    uint256[1] time_buffer;
    bool[1] time_valid;
    uint64 time_current;
    uint256[1] sender_buffer;
    bool[1] sender_valid;
    uint64 sender_current;
    uint256[1] value_buffer;
    bool[1] value_valid;
    uint64 value_current;
    uint256[1] amount_buffer;
    bool[1] amount_valid;
    uint64 amount_current;
    bool[1] refund_buffer;
    bool[1] refund_valid;
    uint64 refund_current;
    bool[1] claim_buffer;
    bool[1] claim_valid;
    uint64 claim_current;
    int256[2] balance_buffer;
    bool[2] balance_valid;
    uint64 balance_current;
    string[1] trigger_0_buffer;
    bool[1] trigger_0_valid;
    uint64 trigger_0_current;
    string[1] trigger_1_buffer;
    bool[1] trigger_1_valid;
    uint64 trigger_1_current;
    string[1] trigger_2_buffer;
    bool[1] trigger_2_valid;
    uint64 trigger_2_current;
    string[1] trigger_3_buffer;
    bool[1] trigger_3_valid;
    uint64 trigger_3_current;
    string[1] trigger_4_buffer;
    bool[1] trigger_4_valid;
    uint64 trigger_4_current;
    string[1] trigger_5_buffer;
    bool[1] trigger_5_valid;
    uint64 trigger_5_current;
    string[1] trigger_6_buffer;
    bool[1] trigger_6_valid;
    uint64 trigger_6_current;
    int256[1] transfer_buffer;
    bool[1] transfer_valid;
    uint64 transfer_current;
    struct BufferpledgeOf {
        int256[2] pledgeOf_buffer;
        bool[2] pledgeOf_valid;
        uint64 pledgeOf_current;
        bool pledgeOf_spawned;
    }
    mapping(uint256 => BufferpledgeOf) pledgeOf_buffer;
    struct pledgeOfParam {
        uint256 user;
    }
    pledgeOfParam[] pledgeOf_params;

    function spawn_pledgeOf(uint256 user) private {
        if (!pledgeOf_buffer[user].pledgeOf_spawned) {
            pledgeOf_params.push(pledgeOfParam({user: user}));
        }
        pledgeOf_buffer[user].pledgeOf_spawned = true;
    }

    function close_pledgeOf(uint256 user) private {
        pledgeOf_buffer[user].pledgeOf_spawned = true;
        for (uint i = 0; i < pledgeOf_params.length; i++) {
            if (pledgeOf_params[i].user == user) {
                delete pledgeOf_params[i];
                return;
            }
        }
    }

    function eval_time(uint256 value) private {
        time_buffer[time_current] = value;
        time_valid[time_current] = true;
    }

    function eval_sender(uint256 value) private {
        sender_buffer[sender_current] = value;
        sender_valid[sender_current] = true;
    }

    function eval_value(uint256 value) private {
        value_buffer[value_current] = value;
        value_valid[value_current] = true;
    }

    function eval_amount(uint256 value) private {
        amount_buffer[amount_current] = value;
        amount_valid[amount_current] = true;
    }

    function eval_refund(bool value) private {
        refund_buffer[refund_current] = value;
        refund_valid[refund_current] = true;
    }

    function eval_claim(bool value) private {
        claim_buffer[claim_current] = value;
        claim_valid[claim_current] = true;
    }

    function eval_balance_0(uint256 amount) private returns (int256 balance) {
        balance = (get_balance(1, int256(0)) + int256(amount));
        balance_buffer[balance_current] = balance;
        balance_valid[balance_current] = true;
    }

    function eval_balance_1(uint256 sender) private returns (int256 balance) {
        balance = (get_balance(1, int256(0)) -
            get_pledgeOf(sender, 0, int256(0)));
        balance_buffer[balance_current] = balance;
        balance_valid[balance_current] = true;
    }

    function eval_transfer_0() private returns (int256 transfer) {
        transfer = get_balance(0, int256(0));
        transfer_buffer[transfer_current] = transfer;
        transfer_valid[transfer_current] = true;
    }

    function eval_transfer_1(uint256 sender) private returns (int256 transfer) {
        transfer = get_pledgeOf(sender, 0, int256(0));
        transfer_buffer[transfer_current] = transfer;
        transfer_valid[transfer_current] = true;
    }

    function eval_pledgeOf_0(
        uint256 user,
        uint256 amount
    ) private returns (int256 pledgeOf) {
        pledgeOf = (get_pledgeOf(user, 1, int256(0)) + int256(amount));
        pledgeOf_buffer[user].pledgeOf_buffer[
            pledgeOf_buffer[user].pledgeOf_current
        ] = pledgeOf;
        pledgeOf_buffer[user].pledgeOf_valid[
            pledgeOf_buffer[user].pledgeOf_current
        ] = true;
    }

    function shift_time() private {
        time_current = (time_current + 1) % 1;
    }

    function shift_sender() private {
        sender_current = (sender_current + 1) % 1;
    }

    function shift_value() private {
        value_current = (value_current + 1) % 1;
    }

    function shift_amount() private {
        amount_current = (amount_current + 1) % 1;
    }

    function shift_refund() private {
        refund_current = (refund_current + 1) % 1;
    }

    function shift_claim() private {
        claim_current = (claim_current + 1) % 1;
    }

    function shift_balance() private {
        balance_current = (balance_current + 1) % 2;
    }

    function shift_trigger_0() private {
        trigger_0_current = (trigger_0_current + 1) % 1;
    }

    function shift_trigger_1() private {
        trigger_1_current = (trigger_1_current + 1) % 1;
    }

    function shift_trigger_2() private {
        trigger_2_current = (trigger_2_current + 1) % 1;
    }

    function shift_trigger_3() private {
        trigger_3_current = (trigger_3_current + 1) % 1;
    }

    function shift_trigger_4() private {
        trigger_4_current = (trigger_4_current + 1) % 1;
    }

    function shift_trigger_5() private {
        trigger_5_current = (trigger_5_current + 1) % 1;
    }

    function shift_trigger_6() private {
        trigger_6_current = (trigger_6_current + 1) % 1;
    }

    function shift_transfer() private {
        transfer_current = (transfer_current + 1) % 1;
    }

    function shift_pledgeOf(uint256 user) private {
        pledgeOf_buffer[user].pledgeOf_current =
            (pledgeOf_buffer[user].pledgeOf_current + 1) %
            2;
    }

    function get_balance(
        uint offset,
        int256 def
    ) private view returns (int256 value) {
        if (balance_valid[(balance_current + offset) % 2]) {
            value = balance_buffer[(balance_current + offset) % 2];
        } else {
            value = def;
        }
    }

    function get_pledgeOf(
        uint256 user,
        uint offset,
        int256 def
    ) private view returns (int256 value) {
        if (
            pledgeOf_buffer[user].pledgeOf_valid[
                (pledgeOf_buffer[user].pledgeOf_current + offset) % 2
            ]
        ) {
            value = pledgeOf_buffer[user].pledgeOf_buffer[
                (pledgeOf_buffer[user].pledgeOf_current + offset) % 2
            ];
        } else {
            value = def;
        }
    }

    event Trigger();
    function trigger() private {
        emit Trigger();
    }

    function bid(uint256 amount) public payable {
        uint256 sender = uint256(uint160(msg.sender));
        uint256 value = uint256(msg.value);
        uint256 time = uint256(block.timestamp);
        shift_time();
        eval_time(time);
        shift_sender();
        eval_sender(sender);
        shift_value();
        eval_value(value);
        shift_amount();
        eval_amount(amount);
        shift_balance();
        spawn_pledgeOf(sender);
        if ((value != amount)) {
            shift_trigger_5();
        }
        if ((time >= uint256(6))) {
            shift_trigger_6();
        }
        for (uint i = 0; i < pledgeOf_params.length; i++) {
            pledgeOfParam memory param = pledgeOf_params[i];
            uint256 user = param.user;
            if ((sender == user)) {
                shift_pledgeOf(user);
            }
        }

        if ((value != amount)) {
            trigger();
        }
        if ((time >= uint256(6))) {
            trigger();
        }
        for (uint i = 0; i < pledgeOf_params.length; i++) {
            pledgeOfParam memory param = pledgeOf_params[i];
            uint256 user = param.user;
            if ((sender == user)) {
                int256 pledgeOf = eval_pledgeOf_0(user, amount);
            }
        }

        int256 balance = eval_balance_0(amount);
    }

    function claim() public {
        bool claim = true;
        uint256 sender = uint256(uint160(msg.sender));
        uint256 time = uint256(block.timestamp);
        shift_time();
        eval_time(time);
        shift_sender();
        eval_sender(sender);
        shift_claim();
        eval_claim(claim);
        shift_transfer();
        spawn_pledgeOf(sender);
        if ((time < uint256(6))) {
            shift_trigger_0();
        }
        if ((sender != uint256(0))) {
            shift_trigger_2();
        }
        if ((time < uint256(6))) {
            trigger();
        }
        if ((sender != uint256(0))) {
            trigger();
        }
        if ((get_balance(0, int256(0)) < int256(100))) {
            shift_trigger_1();
        }
        int256 transfer = eval_transfer_0();
        if ((get_balance(0, int256(0)) < int256(100))) {
            trigger();
        }
    }

    function refund() public {
        bool refund = true;
        uint256 sender = uint256(uint160(msg.sender));
        uint256 time = uint256(block.timestamp);
        shift_time();
        eval_time(time);
        shift_sender();
        eval_sender(sender);
        shift_refund();
        eval_refund(refund);
        shift_balance();
        shift_transfer();
        spawn_pledgeOf(sender);
        if ((time < uint256(6))) {
            shift_trigger_3();
        }
        if ((time < uint256(6))) {
            trigger();
        }
        int256 balance = eval_balance_1(sender);
        if (
            ((get_balance(0, int256(0)) + get_pledgeOf(sender, 0, int256(0))) >=
                int256(100))
        ) {
            shift_trigger_4();
        }
        int256 transfer = eval_transfer_1(sender);
        if (
            ((get_balance(0, int256(0)) + get_pledgeOf(sender, 0, int256(0))) >=
                int256(100))
        ) {
            trigger();
        }
        for (uint i = 0; i < pledgeOf_params.length; i++) {
            pledgeOfParam memory param = pledgeOf_params[i];
            uint256 user = param.user;
            if ((sender == user)) {
                close_pledgeOf(user);
            }
        }
    }
}
