| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.926x | 0.734x | 1.217x |
| materialize | 0.993x | 0.885x | 1.073x |
| perft_bulk_d4 | 0.942x | 0.730x | 1.225x |
| perft_full_d3 | 1.004x | 0.912x | 1.313x |
| fen_parse | 1.149x | 0.905x | 1.251x |
| fen_write_native_ep | 0.977x | 0.726x | 1.772x |
| snapshot | 1.079x | 0.723x | 0.949x |
| san_parse_all | 1.064x | 0.828x | 1.714x |
| san_render_native | 1.098x | 0.964x | 2.696x |
| apply_children_full | 0.971x | 0.789x | 0.878x |
| make_unmake_full | 0.899x | 0.655x | 1.077x |
| startpos | 1.024x | 1.024x | 1.271x |
| chess960_materialize | 1.161x | 1.143x | 1.375x |
| chess960_perft_d3 | 1.094x | 0.980x | 1.359x |
| polyglot_key | 1.056x | 1.054x | 40.294x |
| polyglot_recompute | 1.107x | 1.051x | 1.041x |
| repetition_key | 1.006x | 0.995x | 19.387x |
| corpus_count | 1.239x | 1.228x | 1.580x |
| corpus_materialize | 1.277x | 0.980x | 1.127x |
| corpus_uci_sorted | 0.936x | 0.919x | 1.391x |
| replay_trusted_native | 0.989x | 0.835x | 0.581x |
| replay_checked_native | 0.999x | 0.956x | 0.890x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.01x).
