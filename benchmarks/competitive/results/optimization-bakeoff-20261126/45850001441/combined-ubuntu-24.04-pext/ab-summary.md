| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.988x | 1.550x |
| materialize | 1.003x | 0.999x | 1.648x |
| perft_bulk_d4 | 1.013x | 1.005x | 1.193x |
| perft_full_d3 | 1.221x | 1.059x | 1.650x |
| fen_parse | 0.932x | 0.840x | 1.039x |
| fen_write_native_ep | 1.004x | 0.976x | 2.171x |
| snapshot | 1.004x | 0.998x | 1.153x |
| san_parse_all | 1.107x | 1.091x | 1.904x |
| san_render_native | 1.003x | 0.998x | 2.118x |
| apply_children_full | 1.004x | 0.994x | 1.300x |
| make_unmake_full | 1.006x | 0.989x | 1.170x |
| startpos | 0.997x | 0.997x | 1.114x |
| chess960_materialize | 0.956x | 0.948x | 1.440x |
| chess960_perft_d3 | 0.997x | 0.995x | 1.467x |
| polyglot_key | 1.007x | 1.003x | 39.521x |
| polyglot_recompute | 1.008x | 1.003x | 0.974x |
| repetition_key | 0.825x | 0.696x | 19.437x |
| corpus_count | 1.072x | 1.020x | 1.131x |
| corpus_materialize | 1.007x | 1.006x | 1.083x |
| corpus_uci_sorted | 1.007x | 1.001x | 1.565x |
| replay_trusted_native | 0.965x | 0.931x | 0.887x |
| replay_checked_native | 1.018x | 1.008x | 1.054x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.84x).
