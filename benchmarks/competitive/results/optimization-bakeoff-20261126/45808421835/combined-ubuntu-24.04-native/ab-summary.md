| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.321x |
| materialize | 1.008x | 0.967x | 1.134x |
| perft_bulk_d4 | 0.997x | 0.973x | 1.247x |
| perft_full_d3 | 0.931x | 0.889x | 0.948x |
| fen_parse | 1.042x | 0.999x | 1.085x |
| fen_write_native_ep | 0.987x | 0.946x | 1.885x |
| snapshot | 0.804x | 0.762x | 1.102x |
| san_parse_all | 1.001x | 0.995x | 2.387x |
| san_render_native | 0.992x | 0.982x | 2.168x |
| apply_children_full | 1.025x | 0.967x | 1.167x |
| make_unmake_full | 1.012x | 0.977x | 1.222x |
| startpos | 0.913x | 0.913x | 1.182x |
| chess960_materialize | 1.014x | 1.007x | 1.364x |
| chess960_perft_d3 | 0.998x | 0.985x | 1.406x |
| polyglot_key | 1.004x | 0.998x | 20.770x |
| polyglot_recompute | 1.003x | 1.001x | 1.029x |
| repetition_key | 1.063x | 1.003x | 19.166x |
| corpus_count | 0.991x | 0.989x | 1.317x |
| corpus_materialize | 1.004x | 1.000x | 1.244x |
| corpus_uci_sorted | 1.000x | 0.990x | 1.684x |
| replay_trusted_native | 1.119x | 1.076x | 0.910x |
| replay_checked_native | 1.019x | 0.992x | 0.958x |

Cold first legal move list: 0.013 ms -> 0.014 ms (0.92x).
