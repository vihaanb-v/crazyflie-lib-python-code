pragma solidity ^0.8.24;
contract Contract {
    int64[1] a_buffer;
    bool[1] a_valid;
    uint64 a_current;
    int64[1] b_buffer;
    bool[1] b_valid;
    uint64 b_current;
    int64[1] d_buffer;
    bool[1] d_valid;
    uint64 d_current;
    struct Bufferc {
        int64[2] c_buffer;
        bool[2] c_valid;
        uint64 c_current;
        bool c_spawned;
    }
    mapping(int64 => Bufferc) c_buffer;
    struct cParam {
        int64 p;
    }
    cParam[] c_params;

    function spawn_c(int64 p) private {
        if (!c_buffer[p].c_spawned) {
            c_params.push(cParam({p: p}));
        }
        c_buffer[p].c_spawned = true;
    }

    function close_c(int64 p) private {
        c_buffer[p].c_spawned = false;
        for (uint i = 0; i < c_params.length; i++) {
            if (c_params[i].p == p) {
                delete c_params[i];
                return;
            }
        }
        for (uint i = 0; i < 2; i++) {
            c_buffer[p].c_valid[i] = false;
        }
    }

    function eval_a(int64 value) private {
        a_buffer[a_current] = value;
        a_valid[a_current] = true;
    }

    function eval_b(int64 value) private {
        b_buffer[b_current] = value;
        b_valid[b_current] = true;
    }

    function eval_d_0(int64 a) private returns (int64 d) {
        d = get_c(a, 0, int64(0));
        d_buffer[d_current] = d;
        d_valid[d_current] = true;
    }

    function eval_c_0(int64 p) private returns (int64 c) {
        c = (get_c(p, 1, int64(0)) + int64(1));
        c_buffer[p].c_buffer[c_buffer[p].c_current] = c;
        c_buffer[p].c_valid[c_buffer[p].c_current] = true;
    }

    function shift_a() private {
        a_current = (a_current + 1) % 1;
    }

    function shift_b() private {
        b_current = (b_current + 1) % 1;
    }

    function shift_d() private {
        d_current = (d_current + 1) % 1;
    }

    function shift_c(int64 p) private {
        c_buffer[p].c_current = (c_buffer[p].c_current + 1) % 2;
    }

    function get_d(
        uint64 offset,
        int64 def
    ) private view returns (int64 value) {
        if (d_valid[(d_current + offset) % 1]) {
            value = d_buffer[(d_current + offset) % 1];
        } else {
            value = def;
        }
    }

    function get_c(
        int64 p,
        uint64 offset,
        int64 def
    ) private view returns (int64 value) {
        if (c_buffer[p].c_valid[(c_buffer[p].c_current + offset) % 2]) {
            value = c_buffer[p].c_buffer[(c_buffer[p].c_current + offset) % 2];
        } else {
            value = def;
        }
    }

    event Verdict(int64 d);
    function func_a(int64 a) public {
        shift_a();
        eval_a(a);
        shift_d();
        if ((a > int64(5))) {
            spawn_c(a);
        }
        for (uint i = 0; i < c_params.length; i++) {
            cParam memory param = c_params[i];
            int64 p = param.p;
            if ((p == a)) {
                shift_c(p);
            }
        }

        for (uint i = 0; i < c_params.length; i++) {
            cParam memory param = c_params[i];
            int64 p = param.p;
            if ((p == a)) {
                int64 c = eval_c_0(p);
            }
        }

        int64 d = eval_d_0(a);
        for (uint i = 0; i < c_params.length; i++) {
            cParam memory param = c_params[i];
            int64 p = param.p;
            if ((a < int64(50))) {
                close_c(p);
            }
        }

        emit Verdict(get_d(0, 0));
    }

    function func_a_b(int64 a, int64 b) public {
        shift_a();
        eval_a(a);
        shift_b();
        eval_b(b);
        shift_d();
        if ((a > int64(5))) {
            spawn_c(a);
        }
        for (uint i = 0; i < c_params.length; i++) {
            cParam memory param = c_params[i];
            int64 p = param.p;
            if ((p == a)) {
                shift_c(p);
            }
        }

        for (uint i = 0; i < c_params.length; i++) {
            cParam memory param = c_params[i];
            int64 p = param.p;
            if ((p == a)) {
                int64 c = eval_c_0(p);
            }
        }

        int64 d = eval_d_0(a);
        for (uint i = 0; i < c_params.length; i++) {
            cParam memory param = c_params[i];
            int64 p = param.p;
            if ((a < int64(50))) {
                close_c(p);
            }
        }

        emit Verdict(get_d(0, 0));
    }

    function func_b(int64 b) public {
        shift_b();
        eval_b(b);
        emit Verdict(get_d(0, 0));
    }
}
