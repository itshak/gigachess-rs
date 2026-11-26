| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.933x | 0.880x | 1.449x |
| materialize | 0.976x | 0.929x | 1.284x |
| perft_bulk_d4 | 0.845x | 0.825x | 1.179x |
| perft_full_d3 | 0.954x | 0.824x | 1.264x |
| fen_parse | 1.000x | 0.976x | 1.227x |
| fen_write_native_ep | 0.947x | 0.906x | 1.516x |
| snapshot | 1.337x | 1.332x | 1.218x |
| san_parse_all | 0.943x | 0.917x | 1.721x |
| san_render_native | 0.797x | 0.740x | 2.139x |
| apply_children_full | 0.840x | 0.722x | 0.972x |
| make_unmake_full | 0.733x | 0.679x | 1.155x |
| startpos | 1.006x | 1.006x | 1.283x |
| chess960_materialize | 0.975x | 0.962x | 1.452x |
| chess960_perft_d3 | 0.877x | 0.808x | 1.314x |
| polyglot_key | 1.055x | 0.954x | 26.519x |
| polyglot_recompute | 1.008x | 1.005x | 1.099x |
| repetition_key | 1.137x | 1.062x | 21.786x |
| corpus_count | 0.930x | 0.920x | 1.282x |
| corpus_materialize | 0.983x | 0.982x | 1.216x |
| corpus_uci_sorted | 1.023x | 1.021x | 1.498x |
| replay_trusted_native | 0.812x | 0.754x | 0.522x |
| replay_checked_native | 0.879x | 0.872x | 0.998x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.03x).
