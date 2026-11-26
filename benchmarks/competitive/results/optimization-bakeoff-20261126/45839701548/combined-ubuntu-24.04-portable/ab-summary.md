| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.960x | 1.222x |
| materialize | 1.011x | 0.979x | 1.368x |
| perft_bulk_d4 | 1.015x | 0.964x | 1.218x |
| perft_full_d3 | 1.072x | 0.986x | 1.134x |
| fen_parse | 1.058x | 1.020x | 1.189x |
| fen_write_native_ep | 1.002x | 0.971x | 1.466x |
| snapshot | 0.989x | 0.958x | 0.992x |
| san_parse_all | 1.017x | 0.977x | 1.739x |
| san_render_native | 0.998x | 0.965x | 2.268x |
| apply_children_full | 1.001x | 0.956x | 1.214x |
| make_unmake_full | 0.981x | 0.944x | 1.258x |
| startpos | 1.021x | 1.021x | 1.285x |
| chess960_materialize | 1.009x | 0.993x | 1.590x |
| chess960_perft_d3 | 0.994x | 0.974x | 1.494x |
| polyglot_key | 0.946x | 0.914x | 31.661x |
| polyglot_recompute | 0.773x | 0.749x | 0.802x |
| repetition_key | 1.184x | 1.123x | 24.857x |
| corpus_count | 0.992x | 0.985x | 1.068x |
| corpus_materialize | 0.997x | 0.996x | 1.168x |
| corpus_uci_sorted | 1.009x | 0.988x | 1.611x |
| replay_trusted_native | 0.946x | 0.907x | 0.624x |
| replay_checked_native | 0.995x | 0.990x | 1.207x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.89x).
