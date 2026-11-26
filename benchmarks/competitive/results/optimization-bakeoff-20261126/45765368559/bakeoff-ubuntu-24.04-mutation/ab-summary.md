| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.987x | 1.148x |
| materialize | 1.004x | 1.000x | 1.368x |
| perft_bulk_d4 | 1.017x | 1.004x | 0.877x |
| perft_full_d3 | 1.015x | 1.003x | 0.611x |
| fen_parse | 0.970x | 0.942x | 1.023x |
| fen_write_native_ep | 1.027x | 1.010x | 1.422x |
| snapshot | 1.251x | 1.234x | 0.781x |
| san_parse_all | 1.010x | 0.992x | 0.900x |
| san_render_native | 1.016x | 1.006x | 1.085x |
| apply_children_full | 1.023x | 1.015x | 0.678x |
| make_unmake_full | 1.036x | 1.019x | 0.781x |
| startpos | 0.954x | 0.954x | 0.655x |
| chess960_materialize | 1.006x | 1.003x | 1.168x |
| chess960_perft_d3 | 1.001x | 0.996x | 0.946x |
| polyglot_key | 0.850x | 0.720x | 35.411x |
| polyglot_recompute | 1.022x | 1.019x | 1.026x |
| repetition_key | 0.987x | 0.978x | 17.349x |
| corpus_count | 1.001x | 0.996x | 1.122x |
| corpus_materialize | 1.003x | 0.996x | 1.083x |
| corpus_uci_sorted | 1.005x | 1.005x | 0.625x |
| replay_trusted_native | 1.058x | 1.030x | 0.523x |
| replay_checked_native | 1.037x | 1.016x | 0.763x |

Cold first legal move list: 162.723 ms → 171.780 ms (0.95x).
