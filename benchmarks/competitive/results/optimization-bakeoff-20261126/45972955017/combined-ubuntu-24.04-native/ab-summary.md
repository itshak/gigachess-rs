| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.997x | 1.509x |
| materialize | 0.888x | 0.844x | 1.439x |
| perft_bulk_d4 | 0.987x | 0.977x | 1.154x |
| perft_full_d3 | 1.003x | 0.996x | 1.353x |
| fen_parse | 0.990x | 0.963x | 1.170x |
| fen_write_native_ep | 0.967x | 0.917x | 1.837x |
| snapshot | 1.000x | 0.990x | 1.151x |
| san_parse_all | 0.997x | 0.983x | 1.881x |
| san_render_native | 1.019x | 1.009x | 2.136x |
| apply_children_full | 1.006x | 0.995x | 1.345x |
| make_unmake_full | 0.980x | 0.954x | 1.172x |
| startpos | 1.106x | 1.106x | 1.114x |
| chess960_materialize | 0.864x | 0.862x | 1.321x |
| chess960_perft_d3 | 1.009x | 1.008x | 1.522x |
| polyglot_key | 1.027x | 0.998x | 42.394x |
| polyglot_recompute | 1.004x | 1.003x | 1.050x |
| repetition_key | 1.230x | 1.068x | 25.165x |
| corpus_count | 1.022x | 1.004x | 1.114x |
| corpus_materialize | 1.016x | 1.013x | 1.070x |
| corpus_uci_sorted | 1.006x | 1.001x | 1.567x |
| replay_trusted_native | 1.026x | 0.996x | 0.883x |
| replay_checked_native | 1.006x | 0.979x | 1.196x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.08x).
