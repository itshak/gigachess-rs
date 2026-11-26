| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.292x |
| materialize | 0.997x | 0.992x | 1.201x |
| perft_bulk_d4 | 1.008x | 1.000x | 1.296x |
| perft_full_d3 | 1.041x | 1.015x | 0.782x |
| fen_parse | 1.008x | 1.003x | 1.245x |
| fen_write_native_ep | 1.002x | 0.995x | 1.058x |
| snapshot | 1.000x | 0.996x | 0.961x |
| san_parse_all | 1.058x | 1.047x | 1.880x |
| san_render_native | 1.031x | 0.988x | 2.480x |
| apply_children_full | 1.015x | 0.987x | 0.830x |
| make_unmake_full | 1.055x | 1.012x | 1.197x |
| startpos | 1.113x | 1.113x | 1.304x |
| chess960_materialize | 1.047x | 0.994x | 1.295x |
| chess960_perft_d3 | 1.040x | 0.989x | 1.284x |
| polyglot_key | 0.987x | 0.976x | 24.925x |
| polyglot_recompute | 1.009x | 1.002x | 1.077x |
| repetition_key | 0.998x | 0.986x | 18.755x |
| corpus_count | 1.000x | 0.999x | 1.433x |
| corpus_materialize | 1.003x | 1.003x | 1.201x |
| corpus_uci_sorted | 0.999x | 0.997x | 0.974x |
| replay_trusted_native | 1.092x | 1.067x | 0.453x |
| replay_checked_native | 1.104x | 1.080x | 0.976x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.08x).
