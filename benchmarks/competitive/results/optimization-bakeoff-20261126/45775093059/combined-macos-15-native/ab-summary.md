| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.175x | 1.034x | 1.256x |
| materialize | 1.104x | 0.973x | 1.070x |
| perft_bulk_d4 | 1.372x | 1.142x | 1.229x |
| perft_full_d3 | 2.403x | 2.070x | 1.253x |
| fen_parse | 1.068x | 1.012x | 1.191x |
| fen_write_native_ep | 1.005x | 0.957x | 1.279x |
| snapshot | 1.350x | 1.170x | 0.937x |
| san_parse_all | 1.857x | 1.679x | 1.907x |
| san_render_native | 1.774x | 1.489x | 2.551x |
| apply_children_full | 1.344x | 1.144x | 0.799x |
| make_unmake_full | 1.318x | 1.203x | 1.083x |
| startpos | 1.360x | 1.360x | 1.201x |
| chess960_materialize | 1.141x | 0.988x | 1.329x |
| chess960_perft_d3 | 1.408x | 1.241x | 1.478x |
| polyglot_key | 1.050x | 0.992x | 37.099x |
| polyglot_recompute | 0.961x | 0.929x | 0.971x |
| repetition_key | 1.157x | 1.094x | 20.069x |
| corpus_count | 1.118x | 1.088x | 1.344x |
| corpus_materialize | 1.025x | 1.004x | 1.118x |
| corpus_uci_sorted | 1.510x | 1.497x | 0.994x |
| replay_trusted_native | 1.367x | 1.231x | 0.454x |
| replay_checked_native | 1.146x | 0.998x | 0.831x |

Cold first legal move list: 137.982 ms -> 0.010 ms (13193.97x).
