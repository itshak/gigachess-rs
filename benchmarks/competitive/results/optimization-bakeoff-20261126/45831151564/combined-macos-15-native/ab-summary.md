| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.892x | 0.699x | 1.217x |
| materialize | 0.988x | 0.724x | 1.112x |
| perft_bulk_d4 | 0.959x | 0.780x | 1.171x |
| perft_full_d3 | 0.986x | 0.886x | 1.399x |
| fen_parse | 0.972x | 0.827x | 1.248x |
| fen_write_native_ep | 1.070x | 0.952x | 1.730x |
| snapshot | 1.081x | 0.976x | 1.063x |
| san_parse_all | 0.959x | 0.866x | 1.681x |
| san_render_native | 0.990x | 0.914x | 2.640x |
| apply_children_full | 1.077x | 0.957x | 1.117x |
| make_unmake_full | 1.029x | 0.855x | 1.258x |
| startpos | 1.243x | 1.243x | 1.189x |
| chess960_materialize | 1.028x | 1.027x | 1.414x |
| chess960_perft_d3 | 1.074x | 1.038x | 1.257x |
| polyglot_key | 1.010x | 0.866x | 37.276x |
| polyglot_recompute | 0.979x | 0.867x | 0.884x |
| repetition_key | 0.999x | 0.993x | 19.845x |
| corpus_count | 0.938x | 0.920x | 1.074x |
| corpus_materialize | 0.983x | 0.909x | 1.105x |
| corpus_uci_sorted | 1.016x | 1.014x | 1.407x |
| replay_trusted_native | 1.193x | 1.159x | 0.684x |
| replay_checked_native | 1.144x | 0.931x | 1.146x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.99x).
