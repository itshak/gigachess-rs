| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.033x | 0.985x | 1.227x |
| materialize | 0.969x | 0.927x | 1.217x |
| perft_bulk_d4 | 1.027x | 0.999x | 1.209x |
| perft_full_d3 | 1.010x | 0.980x | 1.154x |
| fen_parse | 0.923x | 0.913x | 1.123x |
| fen_write_native_ep | 1.007x | 0.993x | 1.720x |
| snapshot | 1.001x | 0.999x | 0.898x |
| san_parse_all | 1.014x | 0.994x | 2.172x |
| san_render_native | 1.000x | 0.991x | 2.291x |
| apply_children_full | 1.002x | 0.995x | 1.135x |
| make_unmake_full | 0.999x | 0.997x | 1.300x |
| startpos | 1.000x | 1.000x | 1.287x |
| chess960_materialize | 1.034x | 0.959x | 1.496x |
| chess960_perft_d3 | 1.050x | 1.005x | 1.491x |
| polyglot_key | 1.004x | 1.004x | 22.096x |
| polyglot_recompute | 1.013x | 1.002x | 0.977x |
| repetition_key | 1.007x | 0.990x | 20.397x |
| corpus_count | 0.943x | 0.928x | 1.160x |
| corpus_materialize | 1.020x | 1.019x | 1.343x |
| corpus_uci_sorted | 0.981x | 0.979x | 1.590x |
| replay_trusted_native | 0.909x | 0.853x | 0.621x |
| replay_checked_native | 0.911x | 0.877x | 0.903x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.20x).
