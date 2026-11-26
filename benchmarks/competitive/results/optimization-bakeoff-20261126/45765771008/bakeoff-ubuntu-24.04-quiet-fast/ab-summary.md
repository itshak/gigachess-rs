| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.992x | 1.194x |
| materialize | 1.086x | 1.017x | 1.411x |
| perft_bulk_d4 | 1.003x | 0.990x | 0.878x |
| perft_full_d3 | 0.994x | 0.980x | 0.598x |
| fen_parse | 0.994x | 0.984x | 1.190x |
| fen_write_native_ep | 1.009x | 0.994x | 1.230x |
| snapshot | 0.738x | 0.736x | 0.623x |
| san_parse_all | 1.065x | 1.026x | 1.063x |
| san_render_native | 1.127x | 1.110x | 1.212x |
| apply_children_full | 1.290x | 1.220x | 0.913x |
| make_unmake_full | 1.072x | 1.014x | 0.820x |
| startpos | 0.945x | 0.945x | 0.700x |
| chess960_materialize | 0.914x | 0.910x | 1.132x |
| chess960_perft_d3 | 0.997x | 0.990x | 0.951x |
| polyglot_key | 0.922x | 0.849x | 32.533x |
| polyglot_recompute | 0.958x | 0.956x | 0.974x |
| repetition_key | 0.983x | 0.968x | 13.552x |
| corpus_count | 1.004x | 1.004x | 1.144x |
| corpus_materialize | 0.999x | 0.998x | 1.070x |
| corpus_uci_sorted | 0.987x | 0.985x | 0.621x |
| replay_trusted_native | 1.165x | 1.161x | 0.601x |
| replay_checked_native | 1.102x | 1.078x | 0.817x |

Cold first legal move list: 142.951 ms → 141.995 ms (1.01x).
