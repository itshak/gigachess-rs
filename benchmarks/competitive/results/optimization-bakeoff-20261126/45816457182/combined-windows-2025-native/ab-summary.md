| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.917x | 1.296x |
| materialize | 0.982x | 0.942x | 1.160x |
| perft_bulk_d4 | 1.017x | 0.944x | 1.232x |
| perft_full_d3 | 0.996x | 0.927x | 1.224x |
| fen_parse | 0.932x | 0.852x | 1.312x |
| fen_write_native_ep | 0.956x | 0.937x | 1.479x |
| snapshot | 0.969x | 0.953x | 1.500x |
| san_parse_all | 0.984x | 0.950x | 1.937x |
| san_render_native | 1.055x | 0.997x | 4.008x |
| apply_children_full | 0.961x | 0.952x | 1.211x |
| make_unmake_full | 0.980x | 0.963x | 1.262x |
| startpos | 1.003x | 1.003x | 1.254x |
| chess960_materialize | 1.045x | 0.990x | 1.275x |
| chess960_perft_d3 | 0.990x | 0.984x | 1.372x |
| polyglot_key | 0.992x | 0.986x | 37.726x |
| polyglot_recompute | 0.969x | 0.926x | 0.949x |
| repetition_key | 0.970x | 0.966x | 23.249x |
| corpus_count | 1.072x | 1.015x | 1.074x |
| corpus_materialize | 1.017x | 1.014x | 1.050x |
| corpus_uci_sorted | 1.005x | 1.002x | 1.350x |
| replay_trusted_native | 1.010x | 1.001x | 0.651x |
| replay_checked_native | 0.975x | 0.951x | 0.962x |

Cold first legal move list: 0.010 ms -> 0.012 ms (0.86x).
