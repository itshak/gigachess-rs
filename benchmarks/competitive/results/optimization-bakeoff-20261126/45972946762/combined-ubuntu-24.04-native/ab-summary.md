| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.994x | 1.491x |
| materialize | 1.070x | 1.030x | 1.665x |
| perft_bulk_d4 | 0.991x | 0.981x | 1.157x |
| perft_full_d3 | 1.001x | 0.997x | 1.506x |
| fen_parse | 0.929x | 0.838x | 1.001x |
| fen_write_native_ep | 1.000x | 0.987x | 2.122x |
| snapshot | 1.000x | 0.995x | 1.155x |
| san_parse_all | 0.998x | 0.978x | 1.872x |
| san_render_native | 1.000x | 0.997x | 2.119x |
| apply_children_full | 1.015x | 1.001x | 1.356x |
| make_unmake_full | 0.995x | 0.978x | 1.184x |
| startpos | 1.000x | 1.000x | 1.119x |
| chess960_materialize | 1.001x | 0.994x | 1.557x |
| chess960_perft_d3 | 1.006x | 0.998x | 1.520x |
| polyglot_key | 0.995x | 0.990x | 39.514x |
| polyglot_recompute | 1.018x | 1.015x | 1.019x |
| repetition_key | 0.956x | 0.921x | 19.125x |
| corpus_count | 1.007x | 1.001x | 1.093x |
| corpus_materialize | 0.995x | 0.992x | 1.052x |
| corpus_uci_sorted | 1.006x | 0.998x | 1.556x |
| replay_trusted_native | 0.970x | 0.958x | 0.853x |
| replay_checked_native | 1.008x | 0.987x | 1.157x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.94x).
