| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.995x | 1.312x |
| materialize | 1.000x | 0.992x | 1.339x |
| perft_bulk_d4 | 0.997x | 0.978x | 1.308x |
| perft_full_d3 | 1.000x | 0.996x | 1.273x |
| fen_parse | 1.054x | 1.034x | 1.344x |
| fen_write_native_ep | 0.966x | 0.930x | 1.649x |
| snapshot | 0.996x | 0.950x | 0.992x |
| san_parse_all | 1.000x | 0.988x | 2.001x |
| san_render_native | 1.008x | 0.993x | 2.263x |
| apply_children_full | 1.001x | 0.976x | 1.372x |
| make_unmake_full | 0.989x | 0.976x | 1.389x |
| startpos | 0.999x | 0.999x | 1.328x |
| chess960_materialize | 0.985x | 0.970x | 1.441x |
| chess960_perft_d3 | 1.001x | 0.999x | 1.556x |
| polyglot_key | 0.908x | 0.849x | 23.347x |
| polyglot_recompute | 0.997x | 0.995x | 1.009x |
| repetition_key | 0.980x | 0.944x | 18.197x |
| corpus_count | 0.989x | 0.981x | 1.289x |
| corpus_materialize | 1.001x | 0.998x | 1.198x |
| corpus_uci_sorted | 1.005x | 1.004x | 1.745x |
| replay_trusted_native | 0.995x | 0.969x | 0.889x |
| replay_checked_native | 0.985x | 0.974x | 1.030x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.79x).
