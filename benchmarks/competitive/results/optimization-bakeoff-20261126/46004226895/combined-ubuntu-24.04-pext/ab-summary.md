| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 1.005x | 1.626x |
| materialize | 1.013x | 1.002x | 1.605x |
| perft_bulk_d4 | 1.055x | 1.030x | 1.251x |
| perft_full_d3 | 1.244x | 1.055x | 1.674x |
| fen_parse | 0.992x | 0.983x | 1.063x |
| fen_write_native_ep | 1.006x | 0.981x | 2.154x |
| snapshot | 1.003x | 0.999x | 1.000x |
| san_parse_all | 1.011x | 0.995x | 1.860x |
| san_render_native | 0.986x | 0.981x | 2.090x |
| apply_children_full | 0.988x | 0.960x | 1.337x |
| make_unmake_full | 0.974x | 0.927x | 1.159x |
| startpos | 0.889x | 0.889x | 0.997x |
| chess960_materialize | 0.927x | 0.910x | 1.455x |
| chess960_perft_d3 | 1.014x | 0.993x | 1.495x |
| polyglot_key | 0.955x | 0.910x | 38.395x |
| polyglot_recompute | 0.963x | 0.954x | 0.973x |
| repetition_key | 0.802x | 0.702x | 19.097x |
| corpus_count | 1.145x | 1.114x | 1.279x |
| corpus_materialize | 1.008x | 1.006x | 1.084x |
| corpus_uci_sorted | 1.015x | 1.013x | 1.581x |
| replay_trusted_native | 1.010x | 0.972x | 0.911x |
| replay_checked_native | 0.980x | 0.970x | 1.183x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.10x).
