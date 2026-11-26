| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.920x | 0.779x | 1.377x |
| materialize | 1.078x | 0.887x | 1.090x |
| perft_bulk_d4 | 1.149x | 0.911x | 1.294x |
| perft_full_d3 | 1.090x | 0.835x | 1.199x |
| fen_parse | 1.321x | 1.065x | 1.575x |
| fen_write_native_ep | 1.105x | 0.927x | 1.765x |
| snapshot | 1.100x | 0.812x | 1.069x |
| san_parse_all | 1.038x | 0.811x | 1.675x |
| san_render_native | 1.116x | 0.878x | 3.100x |
| apply_children_full | 1.003x | 0.799x | 1.037x |
| make_unmake_full | 0.894x | 0.735x | 1.234x |
| startpos | 0.783x | 0.783x | 0.930x |
| chess960_materialize | 1.071x | 1.018x | 1.377x |
| chess960_perft_d3 | 1.072x | 1.068x | 1.629x |
| polyglot_key | 0.914x | 0.849x | 40.091x |
| polyglot_recompute | 0.874x | 0.670x | 0.892x |
| repetition_key | 0.855x | 0.639x | 17.161x |
| corpus_count | 1.004x | 0.953x | 1.133x |
| corpus_materialize | 0.993x | 0.879x | 1.228x |
| corpus_uci_sorted | 0.893x | 0.739x | 1.302x |
| replay_trusted_native | 0.942x | 0.820x | 0.717x |
| replay_checked_native | 0.997x | 0.889x | 1.326x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.14x).
