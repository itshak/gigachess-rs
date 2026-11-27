| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 0.985x | 1.597x |
| materialize | 1.008x | 0.973x | 1.392x |
| perft_bulk_d4 | 1.075x | 1.028x | 1.258x |
| perft_full_d3 | 1.217x | 1.067x | 1.633x |
| fen_parse | 1.200x | 1.052x | 1.172x |
| fen_write_native_ep | 1.037x | 0.993x | 1.638x |
| snapshot | 1.140x | 1.015x | 1.139x |
| san_parse_all | 1.080x | 1.034x | 1.769x |
| san_render_native | 1.042x | 0.974x | 4.178x |
| apply_children_full | 1.028x | 0.997x | 1.335x |
| make_unmake_full | 0.980x | 0.891x | 1.156x |
| startpos | 1.021x | 1.021x | 0.988x |
| chess960_materialize | 1.019x | 1.014x | 1.307x |
| chess960_perft_d3 | 1.042x | 1.016x | 1.462x |
| polyglot_key | 0.992x | 0.964x | 37.723x |
| polyglot_recompute | 1.018x | 1.016x | 0.988x |
| repetition_key | 0.805x | 0.795x | 26.885x |
| corpus_count | 1.170x | 1.077x | 1.134x |
| corpus_materialize | 1.004x | 0.995x | 1.084x |
| corpus_uci_sorted | 1.063x | 1.006x | 1.540x |
| replay_trusted_native | 1.043x | 1.040x | 0.852x |
| replay_checked_native | 0.999x | 0.995x | 1.196x |

Cold first legal move list: 0.015 ms -> 0.021 ms (0.74x).
