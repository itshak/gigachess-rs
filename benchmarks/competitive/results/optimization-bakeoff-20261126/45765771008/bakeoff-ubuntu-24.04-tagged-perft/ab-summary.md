| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 1.000x | 1.188x |
| materialize | 0.978x | 0.841x | 1.300x |
| perft_bulk_d4 | 1.056x | 1.038x | 0.922x |
| perft_full_d3 | 0.996x | 0.981x | 0.600x |
| fen_parse | 0.929x | 0.815x | 1.040x |
| fen_write_native_ep | 1.002x | 0.995x | 1.341x |
| snapshot | 0.957x | 0.955x | 0.775x |
| san_parse_all | 1.051x | 1.037x | 0.920x |
| san_render_native | 1.085x | 1.061x | 1.052x |
| apply_children_full | 0.995x | 0.991x | 0.658x |
| make_unmake_full | 0.990x | 0.959x | 0.754x |
| startpos | 1.715x | 1.715x | 0.668x |
| chess960_materialize | 1.009x | 1.006x | 1.248x |
| chess960_perft_d3 | 1.042x | 1.038x | 0.989x |
| polyglot_key | 0.958x | 0.919x | 36.449x |
| polyglot_recompute | 0.951x | 0.950x | 0.977x |
| repetition_key | 1.003x | 1.001x | 13.804x |
| corpus_count | 1.008x | 1.003x | 1.131x |
| corpus_materialize | 1.001x | 1.001x | 1.068x |
| corpus_uci_sorted | 1.007x | 1.005x | 0.628x |
| replay_trusted_native | 1.030x | 1.013x | 0.532x |
| replay_checked_native | 1.000x | 0.982x | 0.738x |

Cold first legal move list: 135.821 ms → 148.608 ms (0.91x).
