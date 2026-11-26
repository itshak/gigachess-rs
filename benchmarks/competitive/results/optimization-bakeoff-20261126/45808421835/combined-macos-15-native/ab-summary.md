| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.844x | 1.273x |
| materialize | 1.086x | 0.994x | 1.060x |
| perft_bulk_d4 | 1.073x | 0.885x | 1.358x |
| perft_full_d3 | 1.045x | 0.878x | 1.270x |
| fen_parse | 1.153x | 1.046x | 1.290x |
| fen_write_native_ep | 1.070x | 0.951x | 1.867x |
| snapshot | 1.054x | 0.894x | 1.026x |
| san_parse_all | 0.944x | 0.755x | 1.773x |
| san_render_native | 0.992x | 0.902x | 2.747x |
| apply_children_full | 0.970x | 0.793x | 1.007x |
| make_unmake_full | 1.005x | 0.912x | 1.193x |
| startpos | 0.730x | 0.730x | 1.116x |
| chess960_materialize | 0.909x | 0.816x | 1.204x |
| chess960_perft_d3 | 1.116x | 0.824x | 1.605x |
| polyglot_key | 1.241x | 1.097x | 36.581x |
| polyglot_recompute | 1.350x | 1.292x | 0.987x |
| repetition_key | 1.308x | 1.163x | 19.833x |
| corpus_count | 1.256x | 1.064x | 1.224x |
| corpus_materialize | 1.173x | 1.105x | 1.201x |
| corpus_uci_sorted | 1.171x | 1.154x | 1.295x |
| replay_trusted_native | 1.234x | 1.139x | 0.700x |
| replay_checked_native | 1.483x | 1.480x | 1.025x |

Cold first legal move list: 0.014 ms -> 0.025 ms (0.55x).
