| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.835x | 1.499x |
| materialize | 1.098x | 0.799x | 1.072x |
| perft_bulk_d4 | 1.062x | 0.986x | 1.234x |
| perft_full_d3 | 1.030x | 0.906x | 1.261x |
| fen_parse | 1.041x | 0.919x | 1.291x |
| fen_write_native_ep | 0.996x | 0.761x | 1.694x |
| snapshot | 0.988x | 0.846x | 0.997x |
| san_parse_all | 1.020x | 0.925x | 1.713x |
| san_render_native | 1.059x | 0.926x | 2.991x |
| apply_children_full | 0.946x | 0.758x | 1.068x |
| make_unmake_full | 0.974x | 0.789x | 1.395x |
| startpos | 1.133x | 1.133x | 1.982x |
| chess960_materialize | 1.164x | 1.155x | 1.522x |
| chess960_perft_d3 | 1.122x | 1.116x | 1.629x |
| polyglot_key | 1.014x | 0.899x | 36.172x |
| polyglot_recompute | 1.057x | 1.045x | 0.904x |
| repetition_key | 1.101x | 1.050x | 20.279x |
| corpus_count | 1.203x | 1.007x | 1.115x |
| corpus_materialize | 1.087x | 1.013x | 1.163x |
| corpus_uci_sorted | 0.962x | 0.953x | 1.399x |
| replay_trusted_native | 1.138x | 1.103x | 0.653x |
| replay_checked_native | 1.289x | 1.211x | 1.322x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.14x).
