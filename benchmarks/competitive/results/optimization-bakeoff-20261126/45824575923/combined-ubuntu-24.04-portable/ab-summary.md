| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.077x | 0.975x | 1.299x |
| materialize | 1.070x | 1.015x | 1.336x |
| perft_bulk_d4 | 1.151x | 1.041x | 1.304x |
| perft_full_d3 | 1.029x | 0.976x | 1.372x |
| fen_parse | 1.035x | 1.010x | 1.215x |
| fen_write_native_ep | 1.005x | 0.992x | 1.702x |
| snapshot | 0.999x | 0.998x | 1.006x |
| san_parse_all | 1.003x | 0.998x | 2.005x |
| san_render_native | 1.005x | 0.994x | 2.290x |
| apply_children_full | 0.982x | 0.969x | 1.340x |
| make_unmake_full | 1.021x | 1.010x | 1.356x |
| startpos | 1.001x | 1.001x | 1.318x |
| chess960_materialize | 1.103x | 1.087x | 1.473x |
| chess960_perft_d3 | 1.065x | 1.062x | 1.541x |
| polyglot_key | 1.005x | 0.999x | 27.930x |
| polyglot_recompute | 1.004x | 1.004x | 1.044x |
| repetition_key | 0.936x | 0.893x | 18.297x |
| corpus_count | 1.096x | 1.059x | 1.289x |
| corpus_materialize | 1.069x | 1.040x | 1.178x |
| corpus_uci_sorted | 1.026x | 1.022x | 1.949x |
| replay_trusted_native | 1.011x | 1.009x | 0.881x |
| replay_checked_native | 1.001x | 0.995x | 1.046x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.04x).
