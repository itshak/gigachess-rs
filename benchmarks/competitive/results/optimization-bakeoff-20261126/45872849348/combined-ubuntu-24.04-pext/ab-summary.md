| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.968x | 1.564x |
| materialize | 1.094x | 1.044x | 1.617x |
| perft_bulk_d4 | 0.962x | 0.940x | 1.146x |
| perft_full_d3 | 1.008x | 0.995x | 1.357x |
| fen_parse | 1.005x | 0.974x | 1.111x |
| fen_write_native_ep | 1.008x | 1.001x | 1.926x |
| snapshot | 1.002x | 0.995x | 1.332x |
| san_parse_all | 0.907x | 0.889x | 1.675x |
| san_render_native | 0.991x | 0.977x | 2.107x |
| apply_children_full | 0.896x | 0.821x | 1.124x |
| make_unmake_full | 0.898x | 0.861x | 1.075x |
| startpos | 1.119x | 1.119x | 1.255x |
| chess960_materialize | 1.074x | 1.070x | 1.417x |
| chess960_perft_d3 | 0.969x | 0.947x | 1.435x |
| polyglot_key | 1.001x | 1.000x | 41.340x |
| polyglot_recompute | 1.026x | 1.024x | 1.064x |
| repetition_key | 0.970x | 0.944x | 24.144x |
| corpus_count | 0.941x | 0.895x | 1.056x |
| corpus_materialize | 1.005x | 1.004x | 1.105x |
| corpus_uci_sorted | 1.005x | 1.001x | 1.539x |
| replay_trusted_native | 1.134x | 0.939x | 0.999x |
| replay_checked_native | 1.019x | 1.018x | 1.187x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.10x).
