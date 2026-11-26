| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.961x | 0.794x | 1.222x |
| materialize | 1.023x | 0.690x | 1.052x |
| perft_bulk_d4 | 1.170x | 0.990x | 1.382x |
| perft_full_d3 | 1.049x | 0.830x | 1.186x |
| fen_parse | 1.063x | 0.945x | 1.128x |
| fen_write_native_ep | 1.003x | 0.692x | 1.772x |
| snapshot | 1.191x | 0.849x | 1.011x |
| san_parse_all | 0.979x | 0.836x | 1.764x |
| san_render_native | 1.083x | 0.926x | 2.891x |
| apply_children_full | 1.084x | 0.766x | 0.900x |
| make_unmake_full | 1.219x | 0.925x | 1.292x |
| startpos | 1.511x | 1.511x | 1.554x |
| chess960_materialize | 1.012x | 0.961x | 1.408x |
| chess960_perft_d3 | 1.164x | 1.122x | 1.697x |
| polyglot_key | 0.903x | 0.893x | 36.930x |
| polyglot_recompute | 1.104x | 1.047x | 1.085x |
| repetition_key | 1.150x | 0.892x | 22.604x |
| corpus_count | 1.095x | 0.807x | 1.395x |
| corpus_materialize | 1.123x | 1.009x | 1.110x |
| corpus_uci_sorted | 1.133x | 1.099x | 1.304x |
| replay_trusted_native | 1.230x | 0.962x | 0.601x |
| replay_checked_native | 1.135x | 0.992x | 1.041x |

Cold first legal move list: 0.021 ms -> 0.021 ms (1.05x).
