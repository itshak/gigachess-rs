| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.000x | 1.374x |
| materialize | 1.023x | 0.974x | 1.433x |
| perft_bulk_d4 | 1.011x | 0.993x | 1.016x |
| perft_full_d3 | 1.018x | 0.956x | 1.496x |
| fen_parse | 1.116x | 1.078x | 1.158x |
| fen_write_native_ep | 0.995x | 0.988x | 1.875x |
| snapshot | 0.710x | 0.706x | 1.079x |
| san_parse_all | 0.981x | 0.968x | 1.839x |
| san_render_native | 1.011x | 0.993x | 2.146x |
| apply_children_full | 0.996x | 0.958x | 1.293x |
| make_unmake_full | 1.040x | 1.014x | 1.204x |
| startpos | 0.815x | 0.815x | 1.156x |
| chess960_materialize | 0.956x | 0.934x | 1.430x |
| chess960_perft_d3 | 0.994x | 0.990x | 1.402x |
| polyglot_key | 1.013x | 1.005x | 41.701x |
| polyglot_recompute | 1.002x | 1.000x | 1.070x |
| repetition_key | 1.116x | 0.814x | 21.345x |
| corpus_count | 1.000x | 1.000x | 0.872x |
| corpus_materialize | 1.006x | 1.005x | 1.034x |
| corpus_uci_sorted | 1.003x | 0.998x | 1.569x |
| replay_trusted_native | 1.019x | 1.003x | 0.893x |
| replay_checked_native | 0.996x | 0.992x | 1.072x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.88x).
