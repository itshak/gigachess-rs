| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.063x | 0.812x | 1.270x |
| materialize | 0.966x | 0.833x | 1.024x |
| perft_bulk_d4 | 1.267x | 0.911x | 1.300x |
| perft_full_d3 | 1.254x | 1.069x | 0.753x |
| fen_parse | 1.038x | 0.862x | 1.224x |
| fen_write_native_ep | 0.876x | 0.707x | 1.267x |
| snapshot | 1.643x | 1.074x | 1.158x |
| san_parse_all | 1.170x | 0.657x | 1.375x |
| san_render_native | 1.609x | 1.193x | 2.669x |
| apply_children_full | 1.114x | 0.930x | 0.794x |
| make_unmake_full | 1.044x | 0.844x | 1.056x |
| startpos | 1.162x | 1.162x | 1.528x |
| chess960_materialize | 1.181x | 1.030x | 1.305x |
| chess960_perft_d3 | 1.439x | 1.181x | 1.647x |
| polyglot_key | 0.904x | 0.834x | 36.905x |
| polyglot_recompute | 0.914x | 0.898x | 1.047x |
| repetition_key | 1.203x | 0.996x | 17.632x |
| corpus_count | 1.132x | 0.938x | 1.234x |
| corpus_materialize | 0.970x | 0.904x | 1.136x |
| corpus_uci_sorted | 1.366x | 1.250x | 0.864x |
| replay_trusted_native | 0.972x | 0.910x | 0.400x |
| replay_checked_native | 1.143x | 1.122x | 0.780x |

Cold first legal move list: 175.239 ms → 0.012 ms (14066.35x).
