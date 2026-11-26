| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.982x | 1.155x |
| materialize | 0.989x | 0.944x | 1.282x |
| perft_bulk_d4 | 0.996x | 0.974x | 1.131x |
| perft_full_d3 | 0.968x | 0.940x | 1.290x |
| fen_parse | 1.104x | 1.014x | 1.361x |
| fen_write_native_ep | 0.961x | 0.887x | 1.484x |
| snapshot | 0.987x | 0.970x | 1.001x |
| san_parse_all | 1.010x | 0.962x | 1.737x |
| san_render_native | 0.976x | 0.940x | 2.336x |
| apply_children_full | 1.043x | 0.987x | 1.168x |
| make_unmake_full | 0.982x | 0.960x | 1.332x |
| startpos | 0.983x | 0.983x | 1.284x |
| chess960_materialize | 1.050x | 1.029x | 1.501x |
| chess960_perft_d3 | 0.991x | 0.988x | 1.389x |
| polyglot_key | 0.965x | 0.946x | 32.078x |
| polyglot_recompute | 0.981x | 0.959x | 1.021x |
| repetition_key | 1.571x | 1.405x | 20.030x |
| corpus_count | 0.994x | 0.990x | 1.021x |
| corpus_materialize | 0.978x | 0.946x | 1.135x |
| corpus_uci_sorted | 1.050x | 0.996x | 1.598x |
| replay_trusted_native | 0.983x | 0.960x | 0.642x |
| replay_checked_native | 0.967x | 0.962x | 1.069x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.02x).
