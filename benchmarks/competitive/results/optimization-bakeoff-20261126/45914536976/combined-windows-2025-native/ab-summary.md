| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.947x | 1.533x |
| materialize | 1.012x | 0.981x | 1.372x |
| perft_bulk_d4 | 0.999x | 0.965x | 1.164x |
| perft_full_d3 | 1.003x | 0.983x | 1.431x |
| fen_parse | 0.870x | 0.839x | 1.052x |
| fen_write_native_ep | 1.001x | 0.983x | 1.690x |
| snapshot | 1.132x | 1.065x | 1.081x |
| san_parse_all | 0.960x | 0.924x | 1.658x |
| san_render_native | 1.037x | 0.988x | 4.118x |
| apply_children_full | 0.994x | 0.969x | 1.325x |
| make_unmake_full | 1.022x | 0.936x | 1.215x |
| startpos | 1.007x | 1.007x | 0.997x |
| chess960_materialize | 1.052x | 1.026x | 1.360x |
| chess960_perft_d3 | 1.040x | 1.021x | 1.481x |
| polyglot_key | 0.983x | 0.963x | 40.752x |
| polyglot_recompute | 0.996x | 0.942x | 0.993x |
| repetition_key | 1.049x | 0.859x | 27.900x |
| corpus_count | 0.992x | 0.978x | 0.943x |
| corpus_materialize | 1.029x | 1.012x | 1.092x |
| corpus_uci_sorted | 1.044x | 1.039x | 1.581x |
| replay_trusted_native | 1.011x | 1.008x | 0.844x |
| replay_checked_native | 1.029x | 1.004x | 1.213x |

Cold first legal move list: 0.019 ms -> 0.015 ms (1.27x).
