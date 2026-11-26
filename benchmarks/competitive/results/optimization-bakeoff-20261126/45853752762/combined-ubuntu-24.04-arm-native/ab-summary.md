| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.558x |
| materialize | 0.995x | 0.989x | 1.312x |
| perft_bulk_d4 | 1.000x | 0.995x | 1.394x |
| perft_full_d3 | 1.001x | 0.998x | 1.332x |
| fen_parse | 0.998x | 0.985x | 1.208x |
| fen_write_native_ep | 0.995x | 0.992x | 1.602x |
| snapshot | 0.991x | 0.990x | 0.954x |
| san_parse_all | 1.000x | 0.993x | 1.833x |
| san_render_native | 0.996x | 0.989x | 2.510x |
| apply_children_full | 0.819x | 0.779x | 0.944x |
| make_unmake_full | 0.945x | 0.913x | 1.441x |
| startpos | 0.997x | 0.997x | 1.327x |
| chess960_materialize | 0.987x | 0.975x | 1.479x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.493x |
| polyglot_key | 0.997x | 0.982x | 25.115x |
| polyglot_recompute | 1.003x | 1.000x | 1.089x |
| repetition_key | 0.996x | 0.992x | 19.042x |
| corpus_count | 1.003x | 0.999x | 1.394x |
| corpus_materialize | 1.002x | 1.000x | 1.226x |
| corpus_uci_sorted | 1.002x | 1.002x | 1.461x |
| replay_trusted_native | 0.970x | 0.945x | 0.613x |
| replay_checked_native | 1.021x | 1.012x | 1.080x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
