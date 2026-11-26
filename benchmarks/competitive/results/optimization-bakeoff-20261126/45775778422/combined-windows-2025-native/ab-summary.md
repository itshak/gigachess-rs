| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.886x | 1.286x |
| materialize | 0.944x | 0.858x | 1.137x |
| perft_bulk_d4 | 0.968x | 0.922x | 1.296x |
| perft_full_d3 | 0.992x | 0.928x | 0.758x |
| fen_parse | 1.045x | 0.959x | 1.262x |
| fen_write_native_ep | 0.965x | 0.927x | 1.060x |
| snapshot | 0.982x | 0.928x | 0.991x |
| san_parse_all | 1.001x | 0.945x | 1.859x |
| san_render_native | 0.969x | 0.892x | 3.490x |
| apply_children_full | 0.964x | 0.863x | 0.838x |
| make_unmake_full | 1.030x | 0.985x | 1.022x |
| startpos | 1.002x | 1.002x | 1.072x |
| chess960_materialize | 1.008x | 0.996x | 1.230x |
| chess960_perft_d3 | 0.990x | 0.979x | 1.327x |
| polyglot_key | 0.987x | 0.978x | 29.427x |
| polyglot_recompute | 0.987x | 0.975x | 1.030x |
| repetition_key | 1.014x | 1.013x | 19.664x |
| corpus_count | 0.990x | 0.987x | 1.416x |
| corpus_materialize | 0.963x | 0.926x | 1.122x |
| corpus_uci_sorted | 0.989x | 0.980x | 0.981x |
| replay_trusted_native | 0.957x | 0.931x | 0.518x |
| replay_checked_native | 0.985x | 0.978x | 0.923x |

Cold first legal move list: 0.032 ms -> 0.029 ms (1.11x).
