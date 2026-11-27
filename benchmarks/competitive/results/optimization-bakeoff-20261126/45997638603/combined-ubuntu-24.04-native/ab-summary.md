| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.079x | 1.031x | 1.572x |
| materialize | 1.025x | 0.967x | 1.498x |
| perft_bulk_d4 | 1.065x | 1.042x | 1.237x |
| perft_full_d3 | 1.255x | 1.054x | 1.714x |
| fen_parse | 0.986x | 0.978x | 1.238x |
| fen_write_native_ep | 1.004x | 0.991x | 2.125x |
| snapshot | 1.000x | 0.992x | 0.997x |
| san_parse_all | 0.907x | 0.878x | 1.606x |
| san_render_native | 0.987x | 0.977x | 2.089x |
| apply_children_full | 1.003x | 0.998x | 1.368x |
| make_unmake_full | 1.000x | 0.974x | 1.192x |
| startpos | 0.893x | 0.893x | 1.000x |
| chess960_materialize | 1.003x | 0.949x | 1.383x |
| chess960_perft_d3 | 1.010x | 0.994x | 1.522x |
| polyglot_key | 1.066x | 1.002x | 40.681x |
| polyglot_recompute | 1.013x | 1.007x | 1.016x |
| repetition_key | 1.021x | 1.005x | 25.326x |
| corpus_count | 1.264x | 1.124x | 1.149x |
| corpus_materialize | 1.006x | 1.002x | 1.063x |
| corpus_uci_sorted | 1.006x | 1.002x | 1.591x |
| replay_trusted_native | 1.003x | 0.994x | 0.840x |
| replay_checked_native | 1.015x | 1.006x | 1.233x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.06x).
