| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.999x | 1.355x |
| materialize | 0.998x | 0.996x | 1.237x |
| perft_bulk_d4 | 1.002x | 0.998x | 1.346x |
| perft_full_d3 | 0.956x | 0.943x | 1.017x |
| fen_parse | 1.003x | 0.996x | 1.242x |
| fen_write_native_ep | 0.999x | 0.995x | 1.565x |
| snapshot | 0.952x | 0.950x | 0.945x |
| san_parse_all | 0.993x | 0.985x | 1.820x |
| san_render_native | 0.978x | 0.968x | 2.478x |
| apply_children_full | 1.005x | 0.960x | 1.048x |
| make_unmake_full | 0.946x | 0.932x | 1.356x |
| startpos | 1.034x | 1.034x | 1.186x |
| chess960_materialize | 0.951x | 0.917x | 1.263x |
| chess960_perft_d3 | 0.956x | 0.924x | 1.267x |
| polyglot_key | 1.006x | 1.000x | 25.786x |
| polyglot_recompute | 1.001x | 1.000x | 1.073x |
| repetition_key | 1.002x | 0.999x | 19.062x |
| corpus_count | 0.997x | 0.988x | 1.462x |
| corpus_materialize | 1.004x | 1.001x | 1.219x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.462x |
| replay_trusted_native | 0.906x | 0.874x | 0.464x |
| replay_checked_native | 0.981x | 0.973x | 0.972x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.14x).
