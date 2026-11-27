| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 0.998x | 1.632x |
| materialize | 0.897x | 0.830x | 1.459x |
| perft_bulk_d4 | 1.058x | 1.032x | 1.269x |
| perft_full_d3 | 1.237x | 1.049x | 1.663x |
| fen_parse | 0.937x | 0.849x | 1.087x |
| fen_write_native_ep | 0.986x | 0.951x | 2.027x |
| snapshot | 1.000x | 0.994x | 1.331x |
| san_parse_all | 1.119x | 1.091x | 1.880x |
| san_render_native | 0.982x | 0.977x | 2.073x |
| apply_children_full | 1.004x | 0.996x | 1.326x |
| make_unmake_full | 0.980x | 0.962x | 1.173x |
| startpos | 0.896x | 0.896x | 1.002x |
| chess960_materialize | 0.862x | 0.861x | 1.296x |
| chess960_perft_d3 | 1.009x | 0.992x | 1.492x |
| polyglot_key | 0.987x | 0.977x | 39.471x |
| polyglot_recompute | 1.051x | 1.046x | 1.007x |
| repetition_key | 1.110x | 0.862x | 22.298x |
| corpus_count | 1.218x | 1.124x | 1.293x |
| corpus_materialize | 1.002x | 0.997x | 1.081x |
| corpus_uci_sorted | 1.012x | 1.012x | 1.558x |
| replay_trusted_native | 1.012x | 0.988x | 0.865x |
| replay_checked_native | 1.011x | 1.002x | 1.156x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.95x).
