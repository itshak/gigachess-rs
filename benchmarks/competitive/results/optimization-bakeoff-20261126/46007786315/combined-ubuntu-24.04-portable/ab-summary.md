| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.036x | 0.925x | 1.378x |
| materialize | 1.000x | 0.983x | 1.344x |
| perft_bulk_d4 | 1.055x | 1.040x | 1.369x |
| perft_full_d3 | 1.174x | 1.016x | 1.498x |
| fen_parse | 1.060x | 1.029x | 1.275x |
| fen_write_native_ep | 1.005x | 0.993x | 1.679x |
| snapshot | 1.002x | 1.000x | 1.010x |
| san_parse_all | 0.999x | 0.983x | 2.046x |
| san_render_native | 1.014x | 0.974x | 2.246x |
| apply_children_full | 0.995x | 0.968x | 1.412x |
| make_unmake_full | 1.001x | 0.986x | 1.404x |
| startpos | 1.002x | 1.002x | 1.328x |
| chess960_materialize | 0.995x | 0.990x | 1.546x |
| chess960_perft_d3 | 1.012x | 0.986x | 1.565x |
| polyglot_key | 0.989x | 0.981x | 23.824x |
| polyglot_recompute | 1.026x | 1.021x | 1.046x |
| repetition_key | 0.942x | 0.912x | 17.925x |
| corpus_count | 1.092x | 1.078x | 1.410x |
| corpus_materialize | 1.003x | 1.001x | 1.204x |
| corpus_uci_sorted | 1.064x | 1.062x | 1.854x |
| replay_trusted_native | 0.985x | 0.979x | 0.876x |
| replay_checked_native | 1.007x | 1.000x | 1.134x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.03x).
