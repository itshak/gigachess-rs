| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.984x | 1.575x |
| materialize | 1.016x | 1.005x | 1.348x |
| perft_bulk_d4 | 1.040x | 1.015x | 1.457x |
| perft_full_d3 | 1.135x | 1.001x | 1.512x |
| fen_parse | 0.991x | 0.981x | 1.204x |
| fen_write_native_ep | 0.996x | 0.982x | 1.598x |
| snapshot | 0.990x | 0.986x | 0.941x |
| san_parse_all | 0.998x | 0.993x | 1.833x |
| san_render_native | 0.998x | 0.992x | 2.658x |
| apply_children_full | 1.001x | 0.971x | 1.160x |
| make_unmake_full | 1.019x | 1.012x | 1.586x |
| startpos | 1.000x | 1.000x | 1.307x |
| chess960_materialize | 1.007x | 1.004x | 1.500x |
| chess960_perft_d3 | 1.016x | 1.006x | 1.521x |
| polyglot_key | 1.004x | 0.995x | 25.613x |
| polyglot_recompute | 1.000x | 0.998x | 1.083x |
| repetition_key | 1.002x | 1.001x | 19.180x |
| corpus_count | 1.097x | 1.065x | 1.507x |
| corpus_materialize | 1.005x | 1.002x | 1.250x |
| corpus_uci_sorted | 1.000x | 0.998x | 1.467x |
| replay_trusted_native | 1.011x | 1.004x | 0.653x |
| replay_checked_native | 1.020x | 1.014x | 1.154x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.81x).
