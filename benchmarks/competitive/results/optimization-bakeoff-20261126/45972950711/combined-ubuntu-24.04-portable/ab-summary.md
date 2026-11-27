| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.991x | 1.277x |
| materialize | 0.914x | 0.839x | 1.612x |
| perft_bulk_d4 | 0.994x | 0.986x | 1.181x |
| perft_full_d3 | 1.007x | 1.004x | 1.321x |
| fen_parse | 1.007x | 0.958x | 1.157x |
| fen_write_native_ep | 1.028x | 0.996x | 1.868x |
| snapshot | 1.000x | 0.991x | 0.998x |
| san_parse_all | 1.004x | 0.971x | 1.923x |
| san_render_native | 1.015x | 0.979x | 2.153x |
| apply_children_full | 0.993x | 0.985x | 1.357x |
| make_unmake_full | 1.072x | 0.995x | 1.264x |
| startpos | 0.993x | 0.993x | 1.358x |
| chess960_materialize | 0.937x | 0.921x | 1.459x |
| chess960_perft_d3 | 1.001x | 0.995x | 1.502x |
| polyglot_key | 1.040x | 0.997x | 42.691x |
| polyglot_recompute | 1.055x | 1.029x | 1.058x |
| repetition_key | 1.233x | 1.139x | 25.746x |
| corpus_count | 0.986x | 0.977x | 1.130x |
| corpus_materialize | 0.997x | 0.992x | 1.132x |
| corpus_uci_sorted | 0.982x | 0.981x | 1.518x |
| replay_trusted_native | 0.968x | 0.940x | 0.838x |
| replay_checked_native | 0.990x | 0.961x | 1.146x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.07x).
