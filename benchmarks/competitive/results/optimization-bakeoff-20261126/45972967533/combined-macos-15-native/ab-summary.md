| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.216x | 0.948x | 1.642x |
| materialize | 1.000x | 0.788x | 1.065x |
| perft_bulk_d4 | 1.138x | 0.941x | 1.295x |
| perft_full_d3 | 1.258x | 1.021x | 1.483x |
| fen_parse | 1.081x | 0.965x | 1.110x |
| fen_write_native_ep | 1.070x | 0.924x | 1.600x |
| snapshot | 0.913x | 0.610x | 0.811x |
| san_parse_all | 1.061x | 0.821x | 1.769x |
| san_render_native | 1.177x | 1.018x | 3.112x |
| apply_children_full | 0.992x | 0.866x | 1.069x |
| make_unmake_full | 1.025x | 0.831x | 1.212x |
| startpos | 0.627x | 0.627x | 0.783x |
| chess960_materialize | 0.872x | 0.757x | 1.537x |
| chess960_perft_d3 | 1.133x | 1.069x | 1.495x |
| polyglot_key | 1.076x | 0.961x | 37.072x |
| polyglot_recompute | 0.879x | 0.871x | 0.910x |
| repetition_key | 1.138x | 0.925x | 19.002x |
| corpus_count | 1.002x | 0.971x | 1.198x |
| corpus_materialize | 1.077x | 1.032x | 1.152x |
| corpus_uci_sorted | 1.138x | 1.075x | 1.399x |
| replay_trusted_native | 0.967x | 0.859x | 0.595x |
| replay_checked_native | 1.244x | 1.077x | 1.219x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.21x).
