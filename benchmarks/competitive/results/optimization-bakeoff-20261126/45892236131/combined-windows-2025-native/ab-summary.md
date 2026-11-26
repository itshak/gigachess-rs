| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.931x | 0.890x | 1.431x |
| materialize | 0.958x | 0.907x | 1.350x |
| perft_bulk_d4 | 0.831x | 0.745x | 0.973x |
| perft_full_d3 | 0.924x | 0.899x | 1.287x |
| fen_parse | 1.035x | 0.991x | 1.280x |
| fen_write_native_ep | 0.939x | 0.895x | 1.625x |
| snapshot | 0.862x | 0.850x | 1.184x |
| san_parse_all | 0.959x | 0.912x | 1.686x |
| san_render_native | 0.892x | 0.837x | 3.825x |
| apply_children_full | 0.861x | 0.761x | 1.168x |
| make_unmake_full | 0.737x | 0.717x | 0.870x |
| startpos | 0.834x | 0.834x | 0.990x |
| chess960_materialize | 0.960x | 0.928x | 1.328x |
| chess960_perft_d3 | 0.853x | 0.817x | 1.276x |
| polyglot_key | 0.999x | 0.977x | 40.846x |
| polyglot_recompute | 0.976x | 0.966x | 0.982x |
| repetition_key | 1.134x | 1.075x | 31.304x |
| corpus_count | 0.933x | 0.918x | 0.976x |
| corpus_materialize | 0.982x | 0.973x | 1.101x |
| corpus_uci_sorted | 0.993x | 0.986x | 1.586x |
| replay_trusted_native | 0.951x | 0.888x | 0.799x |
| replay_checked_native | 0.873x | 0.857x | 1.028x |

Cold first legal move list: 0.013 ms -> 0.017 ms (0.81x).
