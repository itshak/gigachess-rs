| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.969x | 0.896x | 1.443x |
| materialize | 0.968x | 0.922x | 1.255x |
| perft_bulk_d4 | 0.793x | 0.740x | 1.091x |
| perft_full_d3 | 0.966x | 0.871x | 1.256x |
| fen_parse | 0.868x | 0.812x | 1.078x |
| fen_write_native_ep | 0.931x | 0.880x | 1.651x |
| snapshot | 0.896x | 0.893x | 1.227x |
| san_parse_all | 0.874x | 0.860x | 1.792x |
| san_render_native | 0.951x | 0.862x | 2.102x |
| apply_children_full | 0.773x | 0.567x | 1.040x |
| make_unmake_full | 0.765x | 0.691x | 0.983x |
| startpos | 1.391x | 1.391x | 1.796x |
| chess960_materialize | 0.933x | 0.908x | 1.363x |
| chess960_perft_d3 | 0.838x | 0.751x | 1.244x |
| polyglot_key | 1.199x | 1.043x | 31.631x |
| polyglot_recompute | 1.005x | 0.998x | 1.063x |
| repetition_key | 1.148x | 1.050x | 23.680x |
| corpus_count | 0.968x | 0.961x | 1.388x |
| corpus_materialize | 0.982x | 0.967x | 1.186x |
| corpus_uci_sorted | 1.036x | 1.036x | 1.787x |
| replay_trusted_native | 0.785x | 0.688x | 0.697x |
| replay_checked_native | 0.831x | 0.777x | 0.958x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.00x).
