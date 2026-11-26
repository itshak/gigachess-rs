| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.989x | 1.239x |
| materialize | 1.083x | 1.042x | 1.083x |
| perft_bulk_d4 | 1.015x | 0.990x | 0.983x |
| perft_full_d3 | 0.969x | 0.892x | 0.594x |
| fen_parse | 0.812x | 0.792x | 0.959x |
| fen_write_native_ep | 1.002x | 0.995x | 1.274x |
| snapshot | 1.075x | 1.069x | 0.841x |
| san_parse_all | 1.033x | 1.011x | 0.978x |
| san_render_native | 1.027x | 1.020x | 1.130x |
| apply_children_full | 0.925x | 0.834x | 0.622x |
| make_unmake_full | 1.009x | 0.987x | 0.840x |
| startpos | 1.063x | 1.063x | 0.764x |
| chess960_materialize | 1.011x | 1.007x | 1.289x |
| chess960_perft_d3 | 1.017x | 1.011x | 1.023x |
| polyglot_key | 1.026x | 0.998x | 22.760x |
| polyglot_recompute | 0.993x | 0.991x | 1.047x |
| repetition_key | 0.996x | 0.994x | 13.021x |
| corpus_count | 0.996x | 0.995x | 1.241x |
| corpus_materialize | 1.017x | 0.996x | 1.204x |
| corpus_uci_sorted | 0.999x | 0.998x | 0.650x |
| replay_trusted_native | 0.949x | 0.927x | 0.509x |
| replay_checked_native | 0.976x | 0.962x | 0.702x |

Cold first legal move list: 165.385 ms → 160.532 ms (1.03x).
