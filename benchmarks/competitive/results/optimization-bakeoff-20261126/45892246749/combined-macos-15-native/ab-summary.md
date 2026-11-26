| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.816x | 0.664x | 1.378x |
| materialize | 0.963x | 0.811x | 1.032x |
| perft_bulk_d4 | 0.842x | 0.731x | 1.073x |
| perft_full_d3 | 0.716x | 0.651x | 1.014x |
| fen_parse | 0.821x | 0.721x | 1.121x |
| fen_write_native_ep | 0.926x | 0.742x | 1.650x |
| snapshot | 1.134x | 0.983x | 1.147x |
| san_parse_all | 0.964x | 0.840x | 1.700x |
| san_render_native | 1.002x | 0.843x | 2.927x |
| apply_children_full | 0.429x | 0.365x | 0.523x |
| make_unmake_full | 0.643x | 0.508x | 0.907x |
| startpos | 2.108x | 2.108x | 2.394x |
| chess960_materialize | 1.230x | 1.230x | 1.550x |
| chess960_perft_d3 | 0.989x | 0.897x | 1.236x |
| polyglot_key | 1.116x | 0.923x | 40.885x |
| polyglot_recompute | 1.059x | 0.939x | 1.024x |
| repetition_key | 1.062x | 1.021x | 21.806x |
| corpus_count | 1.028x | 0.874x | 1.073x |
| corpus_materialize | 1.068x | 0.973x | 1.130x |
| corpus_uci_sorted | 1.023x | 1.022x | 1.389x |
| replay_trusted_native | 0.839x | 0.802x | 0.607x |
| replay_checked_native | 0.908x | 0.792x | 1.170x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
