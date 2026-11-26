| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.914x | 1.319x |
| materialize | 0.990x | 0.943x | 1.137x |
| perft_bulk_d4 | 0.962x | 0.915x | 1.207x |
| perft_full_d3 | 0.993x | 0.955x | 0.998x |
| fen_parse | 1.044x | 1.014x | 1.078x |
| fen_write_native_ep | 1.003x | 0.997x | 1.825x |
| snapshot | 0.998x | 0.991x | 1.247x |
| san_parse_all | 1.022x | 0.995x | 2.187x |
| san_render_native | 0.970x | 0.915x | 2.067x |
| apply_children_full | 1.155x | 1.073x | 1.282x |
| make_unmake_full | 0.982x | 0.928x | 1.195x |
| startpos | 1.000x | 1.000x | 1.199x |
| chess960_materialize | 1.065x | 0.997x | 1.487x |
| chess960_perft_d3 | 1.057x | 0.992x | 1.496x |
| polyglot_key | 1.010x | 1.000x | 20.486x |
| polyglot_recompute | 0.953x | 0.941x | 0.982x |
| repetition_key | 0.808x | 0.778x | 15.481x |
| corpus_count | 0.890x | 0.801x | 1.181x |
| corpus_materialize | 0.977x | 0.971x | 1.172x |
| corpus_uci_sorted | 0.982x | 0.976x | 1.654x |
| replay_trusted_native | 1.355x | 1.075x | 1.074x |
| replay_checked_native | 1.061x | 1.035x | 1.009x |

Cold first legal move list: 0.013 ms -> 0.006 ms (2.07x).
