| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.981x | 0.802x | 1.312x |
| materialize | 1.080x | 1.045x | 1.092x |
| perft_bulk_d4 | 0.820x | 0.719x | 1.063x |
| perft_full_d3 | 0.878x | 0.862x | 0.966x |
| fen_parse | 1.222x | 1.192x | 1.492x |
| fen_write_native_ep | 0.994x | 0.989x | 1.591x |
| snapshot | 0.819x | 0.794x | 0.976x |
| san_parse_all | 0.928x | 0.874x | 1.771x |
| san_render_native | 0.843x | 0.791x | 2.916x |
| apply_children_full | 0.663x | 0.572x | 0.671x |
| make_unmake_full | 0.636x | 0.553x | 0.799x |
| startpos | 0.407x | 0.407x | 0.476x |
| chess960_materialize | 1.036x | 1.014x | 1.235x |
| chess960_perft_d3 | 0.833x | 0.760x | 1.269x |
| polyglot_key | 0.999x | 0.998x | 20.292x |
| polyglot_recompute | 0.996x | 0.992x | 0.986x |
| repetition_key | 0.998x | 0.997x | 16.696x |
| corpus_count | 1.002x | 0.994x | 1.327x |
| corpus_materialize | 0.999x | 0.992x | 1.157x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.512x |
| replay_trusted_native | 0.793x | 0.639x | 0.847x |
| replay_checked_native | 0.834x | 0.779x | 0.789x |

Cold first legal move list: 0.037 ms -> 0.029 ms (1.28x).
