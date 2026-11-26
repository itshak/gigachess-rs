| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.955x | 0.760x | 1.492x |
| materialize | 0.996x | 0.963x | 1.083x |
| perft_bulk_d4 | 0.964x | 0.914x | 1.208x |
| perft_full_d3 | 0.958x | 0.905x | 1.136x |
| fen_parse | 0.999x | 0.977x | 1.211x |
| fen_write_native_ep | 1.013x | 0.908x | 1.769x |
| snapshot | 1.000x | 0.941x | 1.012x |
| san_parse_all | 0.942x | 0.889x | 1.733x |
| san_render_native | 0.994x | 0.963x | 2.612x |
| apply_children_full | 0.810x | 0.602x | 0.931x |
| make_unmake_full | 0.856x | 0.698x | 1.140x |
| startpos | 1.031x | 1.031x | 1.223x |
| chess960_materialize | 0.970x | 0.946x | 1.517x |
| chess960_perft_d3 | 0.909x | 0.879x | 1.451x |
| polyglot_key | 0.938x | 0.882x | 37.996x |
| polyglot_recompute | 0.978x | 0.921x | 0.936x |
| repetition_key | 1.003x | 0.969x | 19.995x |
| corpus_count | 0.987x | 0.919x | 1.147x |
| corpus_materialize | 1.023x | 0.949x | 1.179x |
| corpus_uci_sorted | 1.107x | 1.072x | 1.383x |
| replay_trusted_native | 1.049x | 0.967x | 0.634x |
| replay_checked_native | 1.022x | 0.964x | 1.119x |

Cold first legal move list: 0.004 ms -> 0.004 ms (0.85x).
