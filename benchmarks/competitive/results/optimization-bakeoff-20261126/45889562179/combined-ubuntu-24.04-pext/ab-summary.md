| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.963x | 0.905x | 1.498x |
| materialize | 1.083x | 1.047x | 1.533x |
| perft_bulk_d4 | 0.973x | 0.925x | 1.158x |
| perft_full_d3 | 1.013x | 0.998x | 1.361x |
| fen_parse | 1.011x | 0.968x | 1.147x |
| fen_write_native_ep | 1.013x | 0.977x | 1.952x |
| snapshot | 0.999x | 0.990x | 1.330x |
| san_parse_all | 0.914x | 0.886x | 1.707x |
| san_render_native | 0.998x | 0.995x | 2.098x |
| apply_children_full | 0.996x | 0.985x | 1.322x |
| make_unmake_full | 0.994x | 0.978x | 1.202x |
| startpos | 1.001x | 1.001x | 0.998x |
| chess960_materialize | 1.013x | 1.001x | 1.427x |
| chess960_perft_d3 | 0.978x | 0.968x | 1.445x |
| polyglot_key | 1.006x | 0.999x | 40.946x |
| polyglot_recompute | 1.011x | 1.007x | 1.046x |
| repetition_key | 0.839x | 0.689x | 19.890x |
| corpus_count | 0.849x | 0.777x | 0.951x |
| corpus_materialize | 0.981x | 0.978x | 1.053x |
| corpus_uci_sorted | 1.007x | 1.002x | 1.549x |
| replay_trusted_native | 1.022x | 1.002x | 0.911x |
| replay_checked_native | 1.001x | 1.000x | 1.139x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.28x).
