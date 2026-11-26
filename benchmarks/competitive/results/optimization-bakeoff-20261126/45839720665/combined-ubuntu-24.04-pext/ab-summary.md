| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.997x | 1.488x |
| materialize | 1.003x | 0.986x | 1.219x |
| perft_bulk_d4 | 0.999x | 0.970x | 1.289x |
| perft_full_d3 | 1.013x | 0.994x | 1.088x |
| fen_parse | 1.113x | 1.023x | 1.248x |
| fen_write_native_ep | 0.996x | 0.973x | 1.518x |
| snapshot | 1.000x | 0.989x | 1.489x |
| san_parse_all | 1.005x | 0.999x | 1.757x |
| san_render_native | 1.023x | 0.965x | 2.243x |
| apply_children_full | 1.052x | 1.006x | 1.386x |
| make_unmake_full | 0.982x | 0.915x | 1.199x |
| startpos | 1.004x | 1.004x | 1.248x |
| chess960_materialize | 1.041x | 1.025x | 1.506x |
| chess960_perft_d3 | 1.005x | 1.001x | 1.511x |
| polyglot_key | 0.993x | 0.989x | 35.712x |
| polyglot_recompute | 0.989x | 0.966x | 1.011x |
| repetition_key | 0.967x | 0.952x | 27.124x |
| corpus_count | 0.996x | 0.996x | 1.174x |
| corpus_materialize | 0.966x | 0.952x | 1.142x |
| corpus_uci_sorted | 0.988x | 0.941x | 1.689x |
| replay_trusted_native | 1.028x | 0.966x | 0.732x |
| replay_checked_native | 1.040x | 1.019x | 1.211x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.11x).
