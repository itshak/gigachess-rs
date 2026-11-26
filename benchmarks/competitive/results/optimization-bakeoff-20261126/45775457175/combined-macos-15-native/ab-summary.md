| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.046x | 0.941x | 1.242x |
| materialize | 1.073x | 0.957x | 0.991x |
| perft_bulk_d4 | 1.136x | 1.022x | 1.227x |
| perft_full_d3 | 1.116x | 1.038x | 0.782x |
| fen_parse | 1.023x | 0.941x | 1.207x |
| fen_write_native_ep | 1.083x | 0.935x | 1.366x |
| snapshot | 1.098x | 0.881x | 0.950x |
| san_parse_all | 1.032x | 0.701x | 1.590x |
| san_render_native | 1.068x | 0.943x | 2.708x |
| apply_children_full | 1.025x | 0.878x | 0.757x |
| make_unmake_full | 1.001x | 0.851x | 1.004x |
| startpos | 0.912x | 0.912x | 1.185x |
| chess960_materialize | 0.953x | 0.942x | 1.250x |
| chess960_perft_d3 | 1.032x | 0.988x | 1.390x |
| polyglot_key | 1.019x | 1.013x | 38.658x |
| polyglot_recompute | 1.042x | 0.993x | 1.041x |
| repetition_key | 0.988x | 0.959x | 18.077x |
| corpus_count | 0.987x | 0.955x | 1.351x |
| corpus_materialize | 0.878x | 0.720x | 1.135x |
| corpus_uci_sorted | 0.888x | 0.823x | 0.920x |
| replay_trusted_native | 0.791x | 0.783x | 0.416x |
| replay_checked_native | 0.837x | 0.798x | 1.006x |

Cold first legal move list: 0.009 ms -> 0.025 ms (0.38x).
