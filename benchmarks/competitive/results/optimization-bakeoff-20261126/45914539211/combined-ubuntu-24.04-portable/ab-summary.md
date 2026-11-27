| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.061x | 1.003x | 1.333x |
| materialize | 0.964x | 0.931x | 1.318x |
| perft_bulk_d4 | 1.087x | 1.034x | 1.349x |
| perft_full_d3 | 1.210x | 0.954x | 1.654x |
| fen_parse | 0.981x | 0.929x | 1.219x |
| fen_write_native_ep | 1.003x | 0.951x | 1.453x |
| snapshot | 0.986x | 0.973x | 1.023x |
| san_parse_all | 0.992x | 0.978x | 1.858x |
| san_render_native | 0.970x | 0.924x | 2.323x |
| apply_children_full | 1.015x | 0.983x | 1.298x |
| make_unmake_full | 0.967x | 0.891x | 1.283x |
| startpos | 0.994x | 0.994x | 1.278x |
| chess960_materialize | 0.989x | 0.985x | 1.609x |
| chess960_perft_d3 | 1.017x | 0.975x | 1.595x |
| polyglot_key | 0.930x | 0.930x | 31.866x |
| polyglot_recompute | 0.973x | 0.937x | 1.014x |
| repetition_key | 0.992x | 0.976x | 20.890x |
| corpus_count | 1.117x | 1.093x | 1.213x |
| corpus_materialize | 1.035x | 1.033x | 1.236x |
| corpus_uci_sorted | 1.008x | 1.008x | 1.751x |
| replay_trusted_native | 1.029x | 1.013x | 0.770x |
| replay_checked_native | 0.876x | 0.833x | 1.128x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.09x).
