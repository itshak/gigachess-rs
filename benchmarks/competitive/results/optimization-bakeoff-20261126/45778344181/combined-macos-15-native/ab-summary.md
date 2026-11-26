| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.978x | 1.310x |
| materialize | 1.014x | 0.964x | 1.034x |
| perft_bulk_d4 | 0.912x | 0.883x | 1.195x |
| perft_full_d3 | 0.956x | 0.883x | 1.283x |
| fen_parse | 0.926x | 0.894x | 1.126x |
| fen_write_native_ep | 1.003x | 0.988x | 1.265x |
| snapshot | 0.994x | 0.958x | 0.973x |
| san_parse_all | 0.985x | 0.955x | 1.771x |
| san_render_native | 1.045x | 0.957x | 2.775x |
| apply_children_full | 1.003x | 0.963x | 0.978x |
| make_unmake_full | 0.850x | 0.718x | 1.039x |
| startpos | 1.012x | 1.012x | 1.185x |
| chess960_materialize | 1.025x | 1.014x | 1.375x |
| chess960_perft_d3 | 0.928x | 0.881x | 1.336x |
| polyglot_key | 1.091x | 1.034x | 38.769x |
| polyglot_recompute | 0.981x | 0.970x | 0.977x |
| repetition_key | 1.059x | 1.056x | 20.119x |
| corpus_count | 1.016x | 1.011x | 1.281x |
| corpus_materialize | 1.027x | 1.023x | 1.138x |
| corpus_uci_sorted | 1.028x | 1.022x | 1.402x |
| replay_trusted_native | 0.950x | 0.845x | 0.537x |
| replay_checked_native | 1.058x | 1.052x | 1.022x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.16x).
