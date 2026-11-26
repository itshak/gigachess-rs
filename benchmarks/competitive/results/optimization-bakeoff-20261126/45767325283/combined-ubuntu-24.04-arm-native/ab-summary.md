| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.193x | 1.152x | 1.293x |
| materialize | 1.028x | 0.999x | 1.208x |
| perft_bulk_d4 | 1.460x | 1.421x | 1.286x |
| perft_full_d3 | 1.254x | 1.179x | 0.757x |
| fen_parse | 1.034x | 1.023x | 1.234x |
| fen_write_native_ep | 0.990x | 0.972x | 1.056x |
| snapshot | 1.217x | 1.205x | 0.914x |
| san_parse_all | 1.939x | 1.862x | 1.780x |
| san_render_native | 2.077x | 1.639x | 2.388x |
| apply_children_full | 1.275x | 1.227x | 0.832x |
| make_unmake_full | 1.270x | 1.236x | 1.152x |
| startpos | 1.553x | 1.553x | 1.185x |
| chess960_materialize | 1.102x | 1.019x | 1.223x |
| chess960_perft_d3 | 1.455x | 1.356x | 1.224x |
| polyglot_key | 1.097x | 0.999x | 26.162x |
| polyglot_recompute | 0.945x | 0.936x | 1.073x |
| repetition_key | 1.311x | 1.117x | 18.658x |
| corpus_count | 1.174x | 1.136x | 1.410x |
| corpus_materialize | 1.070x | 1.070x | 1.206x |
| corpus_uci_sorted | 1.702x | 1.702x | 0.977x |
| replay_trusted_native | 1.278x | 1.222x | 0.417x |
| replay_checked_native | 1.343x | 1.299x | 0.883x |

Cold first legal move list: 153.994 ms → 0.012 ms (12557.65x).
