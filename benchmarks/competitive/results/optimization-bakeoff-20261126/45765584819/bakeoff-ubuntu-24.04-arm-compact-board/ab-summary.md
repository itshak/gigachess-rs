| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.993x | 1.083x |
| materialize | 1.007x | 1.000x | 1.177x |
| perft_bulk_d4 | 1.042x | 1.021x | 0.930x |
| perft_full_d3 | 1.058x | 1.041x | 0.649x |
| fen_parse | 0.995x | 0.989x | 1.196x |
| fen_write_native_ep | 0.989x | 0.985x | 1.048x |
| snapshot | 1.229x | 1.219x | 0.936x |
| san_parse_all | 1.043x | 1.032x | 0.962x |
| san_render_native | 1.070x | 1.060x | 1.217x |
| apply_children_full | 1.145x | 1.133x | 0.739x |
| make_unmake_full | 1.078x | 1.064x | 0.961x |
| startpos | 1.403x | 1.403x | 1.147x |
| chess960_materialize | 1.006x | 0.999x | 1.113x |
| chess960_perft_d3 | 1.061x | 1.045x | 0.893x |
| polyglot_key | 1.129x | 0.999x | 25.627x |
| polyglot_recompute | 0.947x | 0.935x | 1.074x |
| repetition_key | 1.258x | 1.053x | 17.490x |
| corpus_count | 1.019x | 1.012x | 1.228x |
| corpus_materialize | 1.031x | 1.026x | 1.180x |
| corpus_uci_sorted | 1.010x | 1.009x | 0.571x |
| replay_trusted_native | 1.044x | 1.041x | 0.333x |
| replay_checked_native | 1.041x | 1.038x | 0.695x |

Cold first legal move list: 154.276 ms → 154.405 ms (1.00x).
