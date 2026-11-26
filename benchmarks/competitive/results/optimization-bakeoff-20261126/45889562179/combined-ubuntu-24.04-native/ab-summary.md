| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.973x | 0.912x | 1.450x |
| materialize | 0.968x | 0.913x | 1.530x |
| perft_bulk_d4 | 0.982x | 0.935x | 1.149x |
| perft_full_d3 | 1.000x | 0.990x | 1.344x |
| fen_parse | 1.000x | 0.995x | 1.154x |
| fen_write_native_ep | 0.999x | 0.980x | 2.055x |
| snapshot | 1.002x | 0.997x | 1.331x |
| san_parse_all | 0.996x | 0.987x | 1.874x |
| san_render_native | 1.010x | 0.999x | 2.126x |
| apply_children_full | 1.001x | 0.999x | 1.253x |
| make_unmake_full | 1.001x | 0.990x | 1.193x |
| startpos | 1.121x | 1.121x | 1.252x |
| chess960_materialize | 0.926x | 0.922x | 1.438x |
| chess960_perft_d3 | 0.980x | 0.975x | 1.485x |
| polyglot_key | 0.989x | 0.979x | 39.339x |
| polyglot_recompute | 1.013x | 1.004x | 1.008x |
| repetition_key | 0.992x | 0.990x | 19.560x |
| corpus_count | 0.803x | 0.688x | 0.869x |
| corpus_materialize | 0.998x | 0.997x | 1.055x |
| corpus_uci_sorted | 0.986x | 0.985x | 1.549x |
| replay_trusted_native | 1.003x | 1.001x | 0.859x |
| replay_checked_native | 1.016x | 1.009x | 1.227x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.34x).
