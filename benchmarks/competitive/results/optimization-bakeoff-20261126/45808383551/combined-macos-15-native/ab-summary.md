| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.038x | 0.825x | 1.220x |
| materialize | 1.110x | 0.934x | 0.962x |
| perft_bulk_d4 | 0.893x | 0.768x | 1.091x |
| perft_full_d3 | 1.146x | 0.934x | 1.380x |
| fen_parse | 1.050x | 0.907x | 1.149x |
| fen_write_native_ep | 1.002x | 0.904x | 1.781x |
| snapshot | 1.087x | 0.736x | 0.942x |
| san_parse_all | 1.113x | 0.888x | 1.933x |
| san_render_native | 1.050x | 0.895x | 2.567x |
| apply_children_full | 1.044x | 0.848x | 1.030x |
| make_unmake_full | 1.098x | 0.971x | 1.208x |
| startpos | 1.409x | 1.409x | 1.172x |
| chess960_materialize | 1.191x | 1.079x | 1.324x |
| chess960_perft_d3 | 0.940x | 0.862x | 1.283x |
| polyglot_key | 1.157x | 1.071x | 40.919x |
| polyglot_recompute | 1.078x | 1.041x | 0.898x |
| repetition_key | 1.079x | 1.024x | 21.482x |
| corpus_count | 0.658x | 0.643x | 0.936x |
| corpus_materialize | 0.826x | 0.797x | 0.970x |
| corpus_uci_sorted | 0.930x | 0.922x | 1.350x |
| replay_trusted_native | 1.073x | 0.929x | 0.685x |
| replay_checked_native | 0.916x | 0.831x | 1.047x |

Cold first legal move list: 0.013 ms -> 0.007 ms (1.88x).
