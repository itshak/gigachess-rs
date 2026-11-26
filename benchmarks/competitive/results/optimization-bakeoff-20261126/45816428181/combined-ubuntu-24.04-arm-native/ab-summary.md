| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.017x | 0.941x | 1.380x |
| materialize | 1.003x | 0.958x | 1.242x |
| perft_bulk_d4 | 0.923x | 0.896x | 1.244x |
| perft_full_d3 | 1.018x | 0.989x | 1.074x |
| fen_parse | 0.987x | 0.984x | 1.225x |
| fen_write_native_ep | 0.999x | 0.998x | 1.611x |
| snapshot | 0.980x | 0.976x | 0.936x |
| san_parse_all | 0.997x | 0.983x | 1.840x |
| san_render_native | 0.990x | 0.984x | 2.531x |
| apply_children_full | 1.141x | 1.088x | 1.276x |
| make_unmake_full | 1.049x | 0.963x | 1.503x |
| startpos | 1.038x | 1.038x | 1.276x |
| chess960_materialize | 1.049x | 1.013x | 1.378x |
| chess960_perft_d3 | 1.029x | 0.973x | 1.347x |
| polyglot_key | 0.984x | 0.984x | 25.243x |
| polyglot_recompute | 0.999x | 0.997x | 1.069x |
| repetition_key | 0.953x | 0.932x | 18.421x |
| corpus_count | 0.836x | 0.797x | 1.210x |
| corpus_materialize | 0.985x | 0.977x | 1.169x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.453x |
| replay_trusted_native | 1.145x | 1.119x | 0.612x |
| replay_checked_native | 1.066x | 1.057x | 1.044x |

Cold first legal move list: 0.011 ms -> 0.007 ms (1.52x).
