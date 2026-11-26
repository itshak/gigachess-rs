| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.946x | 0.907x | 1.470x |
| materialize | 0.989x | 0.964x | 1.307x |
| perft_bulk_d4 | 0.957x | 0.941x | 1.336x |
| perft_full_d3 | 0.885x | 0.844x | 1.190x |
| fen_parse | 0.979x | 0.959x | 1.198x |
| fen_write_native_ep | 0.940x | 0.907x | 1.499x |
| snapshot | 1.066x | 1.059x | 0.978x |
| san_parse_all | 0.978x | 0.975x | 1.805x |
| san_render_native | 0.988x | 0.979x | 2.642x |
| apply_children_full | 1.030x | 0.974x | 1.189x |
| make_unmake_full | 0.951x | 0.908x | 1.482x |
| startpos | 0.857x | 0.857x | 1.078x |
| chess960_materialize | 0.979x | 0.975x | 1.465x |
| chess960_perft_d3 | 0.954x | 0.949x | 1.433x |
| polyglot_key | 1.012x | 1.000x | 25.797x |
| polyglot_recompute | 1.000x | 0.996x | 1.091x |
| repetition_key | 1.065x | 1.009x | 20.075x |
| corpus_count | 0.955x | 0.950x | 1.328x |
| corpus_materialize | 0.992x | 0.990x | 1.224x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.461x |
| replay_trusted_native | 0.920x | 0.884x | 0.584x |
| replay_checked_native | 0.948x | 0.938x | 1.076x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.08x).
