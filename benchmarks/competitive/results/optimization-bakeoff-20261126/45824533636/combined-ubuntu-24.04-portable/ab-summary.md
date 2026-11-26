| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.983x | 1.148x |
| materialize | 1.013x | 0.988x | 1.283x |
| perft_bulk_d4 | 0.945x | 0.891x | 1.090x |
| perft_full_d3 | 0.975x | 0.929x | 1.283x |
| fen_parse | 1.014x | 0.993x | 1.259x |
| fen_write_native_ep | 0.983x | 0.969x | 1.524x |
| snapshot | 0.999x | 0.992x | 1.001x |
| san_parse_all | 0.995x | 0.987x | 1.731x |
| san_render_native | 0.932x | 0.900x | 2.160x |
| apply_children_full | 0.978x | 0.946x | 1.134x |
| make_unmake_full | 1.002x | 0.975x | 1.316x |
| startpos | 0.984x | 0.984x | 1.283x |
| chess960_materialize | 0.975x | 0.967x | 1.462x |
| chess960_perft_d3 | 0.992x | 0.990x | 1.382x |
| polyglot_key | 1.028x | 1.002x | 33.078x |
| polyglot_recompute | 1.014x | 1.001x | 1.021x |
| repetition_key | 0.635x | 0.571x | 12.703x |
| corpus_count | 1.006x | 0.987x | 1.025x |
| corpus_materialize | 1.022x | 1.014x | 1.146x |
| corpus_uci_sorted | 1.045x | 1.039x | 1.563x |
| replay_trusted_native | 0.997x | 0.973x | 0.662x |
| replay_checked_native | 1.024x | 1.019x | 1.217x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.33x).
