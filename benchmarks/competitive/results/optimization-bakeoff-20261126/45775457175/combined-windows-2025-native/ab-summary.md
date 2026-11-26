| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.970x | 0.921x | 1.394x |
| materialize | 0.980x | 0.952x | 1.296x |
| perft_bulk_d4 | 0.950x | 0.901x | 1.212x |
| perft_full_d3 | 0.921x | 0.826x | 0.811x |
| fen_parse | 0.956x | 0.919x | 1.119x |
| fen_write_native_ep | 0.971x | 0.936x | 1.326x |
| snapshot | 0.840x | 0.794x | 1.045x |
| san_parse_all | 0.948x | 0.882x | 1.576x |
| san_render_native | 0.991x | 0.926x | 4.082x |
| apply_children_full | 0.995x | 0.921x | 0.941x |
| make_unmake_full | 0.980x | 0.946x | 0.948x |
| startpos | 1.024x | 1.024x | 1.092x |
| chess960_materialize | 0.989x | 0.965x | 1.281x |
| chess960_perft_d3 | 0.995x | 0.986x | 1.357x |
| polyglot_key | 1.296x | 1.285x | 44.276x |
| polyglot_recompute | 0.967x | 0.965x | 0.953x |
| repetition_key | 0.978x | 0.816x | 25.197x |
| corpus_count | 0.968x | 0.952x | 1.245x |
| corpus_materialize | 1.005x | 0.971x | 1.102x |
| corpus_uci_sorted | 0.971x | 0.960x | 0.965x |
| replay_trusted_native | 0.972x | 0.934x | 0.558x |
| replay_checked_native | 0.964x | 0.899x | 0.942x |

Cold first legal move list: 0.033 ms -> 0.031 ms (1.06x).
