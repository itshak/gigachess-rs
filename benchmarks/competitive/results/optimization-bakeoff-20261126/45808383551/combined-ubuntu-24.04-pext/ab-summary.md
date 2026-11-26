| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.937x | 0.891x | 1.390x |
| materialize | 0.912x | 0.837x | 1.455x |
| perft_bulk_d4 | 0.830x | 0.711x | 1.042x |
| perft_full_d3 | 0.954x | 0.907x | 1.533x |
| fen_parse | 0.959x | 0.945x | 1.071x |
| fen_write_native_ep | 0.957x | 0.922x | 1.853x |
| snapshot | 0.647x | 0.644x | 0.998x |
| san_parse_all | 0.899x | 0.881x | 1.859x |
| san_render_native | 1.026x | 1.016x | 2.160x |
| apply_children_full | 1.003x | 0.989x | 1.208x |
| make_unmake_full | 0.995x | 0.982x | 1.106x |
| startpos | 0.806x | 0.806x | 1.327x |
| chess960_materialize | 1.009x | 0.981x | 1.477x |
| chess960_perft_d3 | 1.003x | 0.976x | 1.394x |
| polyglot_key | 1.001x | 0.998x | 40.505x |
| polyglot_recompute | 1.003x | 1.000x | 1.050x |
| repetition_key | 1.103x | 0.918x | 21.758x |
| corpus_count | 0.658x | 0.588x | 0.925x |
| corpus_materialize | 0.950x | 0.939x | 1.046x |
| corpus_uci_sorted | 0.988x | 0.981x | 1.563x |
| replay_trusted_native | 0.992x | 0.988x | 0.810x |
| replay_checked_native | 0.957x | 0.945x | 1.113x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.60x).
