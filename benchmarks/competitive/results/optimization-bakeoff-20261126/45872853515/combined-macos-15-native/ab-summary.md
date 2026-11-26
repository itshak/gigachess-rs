| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.926x | 0.590x | 1.426x |
| materialize | 0.996x | 0.872x | 1.157x |
| perft_bulk_d4 | 1.033x | 0.844x | 1.170x |
| perft_full_d3 | 1.222x | 0.925x | 1.522x |
| fen_parse | 1.092x | 0.845x | 1.292x |
| fen_write_native_ep | 1.026x | 0.777x | 1.809x |
| snapshot | 1.182x | 0.700x | 0.932x |
| san_parse_all | 1.079x | 0.880x | 1.719x |
| san_render_native | 1.142x | 0.783x | 2.934x |
| apply_children_full | 1.013x | 0.744x | 1.022x |
| make_unmake_full | 1.074x | 0.791x | 1.280x |
| startpos | 1.225x | 1.225x | 1.184x |
| chess960_materialize | 0.664x | 0.548x | 1.578x |
| chess960_perft_d3 | 0.721x | 0.715x | 1.333x |
| polyglot_key | 0.696x | 0.551x | 34.698x |
| polyglot_recompute | 0.858x | 0.768x | 1.163x |
| repetition_key | 0.931x | 0.882x | 18.911x |
| corpus_count | 0.989x | 0.734x | 1.264x |
| corpus_materialize | 0.730x | 0.562x | 1.027x |
| corpus_uci_sorted | 0.744x | 0.627x | 1.321x |
| replay_trusted_native | 0.746x | 0.730x | 0.651x |
| replay_checked_native | 0.728x | 0.699x | 1.051x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.97x).
