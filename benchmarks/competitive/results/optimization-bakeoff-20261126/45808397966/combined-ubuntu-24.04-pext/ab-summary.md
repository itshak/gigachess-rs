| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.974x | 0.904x | 1.446x |
| materialize | 1.071x | 0.993x | 1.522x |
| perft_bulk_d4 | 0.885x | 0.787x | 1.111x |
| perft_full_d3 | 0.977x | 0.943x | 1.446x |
| fen_parse | 0.985x | 0.952x | 1.155x |
| fen_write_native_ep | 1.007x | 0.956x | 1.933x |
| snapshot | 1.003x | 0.992x | 1.153x |
| san_parse_all | 0.982x | 0.975x | 2.025x |
| san_render_native | 0.996x | 0.984x | 2.138x |
| apply_children_full | 1.021x | 1.010x | 1.210x |
| make_unmake_full | 0.992x | 0.978x | 1.100x |
| startpos | 1.003x | 1.003x | 1.133x |
| chess960_materialize | 1.090x | 1.058x | 1.577x |
| chess960_perft_d3 | 1.030x | 0.985x | 1.430x |
| polyglot_key | 1.013x | 1.002x | 41.549x |
| polyglot_recompute | 0.964x | 0.954x | 1.040x |
| repetition_key | 0.859x | 0.774x | 17.098x |
| corpus_count | 0.686x | 0.614x | 0.985x |
| corpus_materialize | 0.989x | 0.976x | 1.092x |
| corpus_uci_sorted | 0.988x | 0.978x | 1.546x |
| replay_trusted_native | 1.003x | 1.002x | 0.815x |
| replay_checked_native | 0.986x | 0.974x | 1.096x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.61x).
