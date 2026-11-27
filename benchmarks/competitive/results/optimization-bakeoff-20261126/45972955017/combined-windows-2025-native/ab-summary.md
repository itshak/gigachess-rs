| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.994x | 1.538x |
| materialize | 1.000x | 0.993x | 1.146x |
| perft_bulk_d4 | 0.998x | 0.964x | 1.428x |
| perft_full_d3 | 1.014x | 0.995x | 1.256x |
| fen_parse | 0.998x | 0.975x | 1.388x |
| fen_write_native_ep | 1.016x | 1.008x | 1.639x |
| snapshot | 1.011x | 0.969x | 1.166x |
| san_parse_all | 0.998x | 0.991x | 2.012x |
| san_render_native | 1.078x | 1.015x | 3.597x |
| apply_children_full | 1.002x | 0.976x | 1.319x |
| make_unmake_full | 0.963x | 0.907x | 1.252x |
| startpos | 1.005x | 1.005x | 1.007x |
| chess960_materialize | 0.997x | 0.986x | 1.344x |
| chess960_perft_d3 | 1.006x | 1.004x | 1.610x |
| polyglot_key | 1.009x | 0.991x | 20.318x |
| polyglot_recompute | 1.014x | 1.013x | 0.991x |
| repetition_key | 0.991x | 0.990x | 18.939x |
| corpus_count | 1.006x | 1.001x | 1.319x |
| corpus_materialize | 0.999x | 0.996x | 1.198x |
| corpus_uci_sorted | 1.002x | 0.997x | 1.526x |
| replay_trusted_native | 0.988x | 0.940x | 1.074x |
| replay_checked_native | 1.016x | 1.002x | 1.135x |

Cold first legal move list: 0.015 ms -> 0.013 ms (1.21x).
