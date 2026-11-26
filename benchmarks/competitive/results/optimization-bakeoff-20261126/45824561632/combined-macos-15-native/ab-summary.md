| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.100x | 0.900x | 1.397x |
| materialize | 0.992x | 0.753x | 1.015x |
| perft_bulk_d4 | 1.076x | 0.950x | 1.151x |
| perft_full_d3 | 0.965x | 0.835x | 1.293x |
| fen_parse | 1.059x | 0.913x | 1.205x |
| fen_write_native_ep | 1.035x | 0.888x | 1.650x |
| snapshot | 0.915x | 0.607x | 0.850x |
| san_parse_all | 0.996x | 0.801x | 1.663x |
| san_render_native | 1.038x | 0.934x | 2.703x |
| apply_children_full | 1.129x | 0.989x | 1.180x |
| make_unmake_full | 1.168x | 1.088x | 1.300x |
| startpos | 1.046x | 1.046x | 1.215x |
| chess960_materialize | 0.967x | 0.962x | 1.373x |
| chess960_perft_d3 | 1.073x | 1.072x | 1.191x |
| polyglot_key | 0.986x | 0.877x | 34.114x |
| polyglot_recompute | 1.033x | 1.014x | 1.050x |
| repetition_key | 0.960x | 0.784x | 18.973x |
| corpus_count | 0.888x | 0.798x | 0.836x |
| corpus_materialize | 1.038x | 1.024x | 1.037x |
| corpus_uci_sorted | 0.899x | 0.886x | 1.403x |
| replay_trusted_native | 1.058x | 1.024x | 0.683x |
| replay_checked_native | 0.922x | 0.872x | 1.025x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.49x).
