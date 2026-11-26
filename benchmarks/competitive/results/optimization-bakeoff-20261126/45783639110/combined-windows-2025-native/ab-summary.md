| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.991x | 1.290x |
| materialize | 1.027x | 1.002x | 1.167x |
| perft_bulk_d4 | 1.044x | 0.994x | 1.333x |
| perft_full_d3 | 0.948x | 0.908x | 1.125x |
| fen_parse | 1.117x | 1.088x | 1.338x |
| fen_write_native_ep | 0.980x | 0.935x | 0.967x |
| snapshot | 1.323x | 1.308x | 1.191x |
| san_parse_all | 1.051x | 1.015x | 2.017x |
| san_render_native | 1.043x | 1.003x | 3.842x |
| apply_children_full | 0.858x | 0.805x | 1.002x |
| make_unmake_full | 1.013x | 0.968x | 1.225x |
| startpos | 1.246x | 1.246x | 1.254x |
| chess960_materialize | 1.080x | 1.040x | 1.219x |
| chess960_perft_d3 | 1.084x | 1.062x | 1.344x |
| polyglot_key | 0.958x | 0.899x | 27.673x |
| polyglot_recompute | 1.068x | 1.063x | 1.041x |
| repetition_key | 1.092x | 1.085x | 20.553x |
| corpus_count | 1.105x | 0.999x | 1.345x |
| corpus_materialize | 1.008x | 0.977x | 1.156x |
| corpus_uci_sorted | 1.025x | 0.981x | 1.382x |
| replay_trusted_native | 1.090x | 1.073x | 0.730x |
| replay_checked_native | 1.043x | 1.004x | 0.987x |

Cold first legal move list: 0.040 ms -> 0.030 ms (1.31x).
